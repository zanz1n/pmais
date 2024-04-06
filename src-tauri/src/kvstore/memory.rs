use super::{KvStore, KvStoreError};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
struct Value {
    ttl: Option<Instant>,
    buf: Vec<u8>,
}

#[derive(Default)]
pub struct MemoryKvStore {
    inner: RwLock<HashMap<String, Value>>,
}

impl MemoryKvStore {
    #[inline]
    pub fn new() -> MemoryKvStore {
        MemoryKvStore {
            inner: RwLock::new(HashMap::with_capacity(128)),
        }
    }
}

impl KvStore for MemoryKvStore {
    async fn get(&self, key: String) -> Result<Option<Vec<u8>>, KvStoreError> {
        let lock = self.inner.read().await;
        let value = lock.get(&key);

        match value {
            Some(v) => {
                if let Some(i) = v.ttl {
                    if Instant::now() > i {
                        drop(lock);
                        let mut lock = self.inner.write().await;
                        lock.remove(&key);

                        return Ok(None);
                    }
                }
                Ok(Some(v.buf.clone()))
            }
            None => Ok(None),
        }
    }

    async fn get_ttl(&self, key: String, ttl: Duration) -> Result<Option<Vec<u8>>, KvStoreError> {
        let lock = self.inner.read().await;

        match lock.get(&key) {
            Some(v) => {
                let mut v = v.clone();
                let b = v.buf.clone();
                drop(lock);
                v.ttl = Some(Instant::now() + ttl);

                let mut lock = self.inner.write().await;
                lock.insert(key, v);

                Ok(Some(b))
            }
            None => Ok(None),
        }
    }

    async fn set(&self, key: String, value: Vec<u8>) -> Result<(), KvStoreError> {
        let mut lock = self.inner.write().await;
        lock.insert(
            key,
            Value {
                ttl: None,
                buf: value,
            },
        );

        Ok(())
    }

    async fn set_ttl(
        &self,
        key: String,
        value: Vec<u8>,
        ttl: Duration,
    ) -> Result<(), KvStoreError> {
        let mut lock = self.inner.write().await;
        lock.insert(
            key,
            Value {
                ttl: Some(Instant::now() + ttl),
                buf: value,
            },
        );

        Ok(())
    }

    async fn delete(&self, key: String) -> Result<(), KvStoreError> {
        let mut lock = self.inner.write().await;
        lock.remove(&key);

        Ok(())
    }
}
