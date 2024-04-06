use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, time::Duration};

mod encoding;
pub mod memory;

#[derive(Debug, thiserror::Error)]
pub enum KvStoreError {
    #[error("Failed to get")]
    GetFailed,
    #[error("Failed to set")]
    SetFailed,
    #[error("Failed to delete")]
    DeleteFailed,
    #[error("The provided cache ttl is invalid")]
    InvalidTtl,
    #[error("Failed to serialize the value")]
    SerializeFailed,
    #[error("Failed to deserialize the value")]
    DeserializeFailed,
}

pub trait KvStore: Send + Sync {
    fn get(
        &self,
        key: String,
    ) -> impl Future<Output = Result<Option<Vec<u8>>, KvStoreError>> + Send;

    fn get_ttl(
        &self,
        key: String,
        ttl: Duration,
    ) -> impl Future<Output = Result<Option<Vec<u8>>, KvStoreError>> + Send;

    fn set(
        &self,
        key: String,
        value: Vec<u8>,
    ) -> impl Future<Output = Result<(), KvStoreError>> + Send;

    fn set_ttl(
        &self,
        key: String,
        value: Vec<u8>,
        ttl: Duration,
    ) -> impl Future<Output = Result<(), KvStoreError>> + Send;

    fn delete(&self, key: String) -> impl Future<Output = Result<(), KvStoreError>> + Send;
}

pub trait KvStoreSerde {
    fn get_de<T: DeserializeOwned + Send>(
        &self,
        key: String,
    ) -> impl Future<Output = Result<Option<T>, KvStoreError>> + Send;

    fn get_ttl_de<T: DeserializeOwned + Send>(
        &self,
        key: String,
        ttl: Duration,
    ) -> impl Future<Output = Result<Option<T>, KvStoreError>> + Send;

    fn set_ser<T: Serialize + Send + Sync>(
        &self,
        key: String,
        value: &T,
    ) -> impl Future<Output = Result<(), KvStoreError>> + Send;

    fn set_ttl_ser<T: Serialize + Send + Sync>(
        &self,
        key: String,
        value: &T,
        ttl: Duration,
    ) -> impl Future<Output = Result<(), KvStoreError>> + Send;
}

impl<S: KvStore> KvStoreSerde for S {
    async fn get_de<T: DeserializeOwned + Send>(
        &self,
        key: String,
    ) -> Result<Option<T>, KvStoreError> {
        self.get(key).await?.map_or(Ok(None), |buf| {
            encoding::deserialize(&buf).ok_or(KvStoreError::DeserializeFailed)
        })
    }

    async fn get_ttl_de<T: DeserializeOwned + Send>(
        &self,
        key: String,
        ttl: Duration,
    ) -> Result<Option<T>, KvStoreError> {
        self.get_ttl(key, ttl).await?.map_or(Ok(None), |buf| {
            encoding::deserialize(&buf).ok_or(KvStoreError::DeserializeFailed)
        })
    }

    async fn set_ser<T: Serialize + Send + Sync>(
        &self,
        key: String,
        value: &T,
    ) -> Result<(), KvStoreError> {
        self.set(
            key,
            encoding::serialize(value).ok_or(KvStoreError::SerializeFailed)?,
        )
        .await
    }

    async fn set_ttl_ser<T: Serialize + Send + Sync>(
        &self,
        key: String,
        value: &T,
        ttl: Duration,
    ) -> Result<(), KvStoreError> {
        self.set_ttl(
            key,
            encoding::serialize(value).ok_or(KvStoreError::SerializeFailed)?,
            ttl,
        )
        .await
    }
}
