use super::{KvStore, KvStoreError};
use chrono::Utc;
use sqlx::{FromRow, Pool, Sqlite};
use std::time::Duration;

#[derive(Debug, Clone, FromRow)]
struct ValueSqlRow {
    value: Vec<u8>,
    ttl: Option<i64>,
}

#[derive(Debug, Clone, FromRow)]
struct BoolSqlRow {
    value: bool,
}

pub struct SqliteKvStore {
    db: Pool<Sqlite>,
}

impl SqliteKvStore {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}

impl KvStore for SqliteKvStore {
    async fn get(&self, key: String) -> Result<Option<Vec<u8>>, KvStoreError> {
        let r = sqlx::query_as::<Sqlite, ValueSqlRow>(
            "SELECT ttl, value FROM key_value WHERE key = $1",
        )
        .bind(&key)
        .fetch_one(&self.db)
        .await;

        match r {
            Ok(v) => {
                if let Some(ttl) = v.ttl {
                    if Utc::now().timestamp_millis() > ttl {
                        _ = self.delete(key).await;
                        return Ok(None);
                    }
                }

                Ok(Some(v.value))
            }
            Err(e) => {
                if let sqlx::Error::RowNotFound = &e {
                    return Ok(None);
                }
                log::error!(
                    target: "SqliteKvStore::get",
                    "Failed to fetch key_value sqlite data: {e}",
                );

                Err(KvStoreError::GetFailed)
            }
        }
    }

    async fn get_ttl(&self, key: String, ttl: Duration) -> Result<Option<Vec<u8>>, KvStoreError> {
        let r = sqlx::query_as::<Sqlite, ValueSqlRow>(
            "UPDATE key_value SET ttl = $1 WHERE key = $2 RETURNING ttl, value",
        )
        .bind((Utc::now() + ttl).timestamp_millis())
        .bind(key)
        .fetch_one(&self.db)
        .await;

        match r {
            Ok(v) => Ok(Some(v.value)),
            Err(e) => {
                if let sqlx::Error::RowNotFound = &e {
                    return Ok(None);
                }
                log::error!(
                    target: "SqliteKvStore::get_ttl",
                    "Failed to update key_value sqlite data: {e}",
                );

                Err(KvStoreError::GetFailed)
            }
        }
    }

    async fn set(&self, key: String, value: Vec<u8>) -> Result<(), KvStoreError> {
        let mut tx = self.db.begin().await.unwrap();

        let BoolSqlRow { value: exists } = sqlx::query_as::<Sqlite, BoolSqlRow>(
            "SELECT EXISTS(SELECT 1 FROM key_value WHERE key = $1) AS value",
        )
        .bind(&key)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            log::error!(
                target: "SqliteKvStore::set",
                "Failed to fetch key_value sqlite data: {e}",
            );
            KvStoreError::SetFailed
        })?;

        let query = if exists {
            sqlx::query("UPDATE key_value SET ttl = NULL, value = $1 WHERE key = $2")
                .bind(value)
                .bind(key)
        } else {
            sqlx::query("INSERT INTO key_value (key, value) VALUES ($1, $2)")
                .bind(key)
                .bind(value)
        };

        query.execute(&mut *tx).await.map_err(|e| {
            log::error!(
                target: "SqliteKvStore::set",
                "Failed to insert key_value sqlite data: {e}",
            );
            KvStoreError::SetFailed
        })?;

        tx.commit().await.map_err(|e| {
            log::error!(
                target: "SqliteKvStore::set",
                "Failed to execute transaction on key_value sqlite data: {e}",
            );
            KvStoreError::SetFailed
        })
    }

    async fn set_ttl(
        &self,
        key: String,
        value: Vec<u8>,
        ttl: Duration,
    ) -> Result<(), KvStoreError> {
        let mut tx = self.db.begin().await.unwrap();

        let BoolSqlRow { value: exists } = sqlx::query_as::<Sqlite, BoolSqlRow>(
            "SELECT EXISTS(SELECT 1 FROM key_value WHERE key = $1) AS value",
        )
        .bind(&key)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            log::error!(
                target: "SqliteKvStore::set_ttl",
                "Failed to fetch key_value sqlite data: {e}",
            );
            KvStoreError::SetFailed
        })?;

        let query = if exists {
            sqlx::query("UPDATE key_value SET ttl = $1, value = $2 WHERE key = $3")
                .bind((Utc::now() + ttl).timestamp_millis())
                .bind(value)
                .bind(key)
        } else {
            sqlx::query("INSERT INTO key_value (key, ttl, value) VALUES ($1, $2, $3)")
                .bind(key)
                .bind((Utc::now() + ttl).timestamp_millis())
                .bind(value)
        };

        query.execute(&mut *tx).await.map_err(|e| {
            log::error!(
                target: "SqliteKvStore::set_ttl",
                "Failed to insert key_value sqlite data: {e}",
            );
            KvStoreError::SetFailed
        })?;

        tx.commit().await.map_err(|e| {
            log::error!(
                target: "SqliteKvStore::set_ttl",
                "Failed to execute transaction on key_value sqlite data: {e}",
            );
            KvStoreError::SetFailed
        })
    }

    async fn delete(&self, key: String) -> Result<(), KvStoreError> {
        sqlx::query("DELETE FROM key_value WHERE key = $1")
            .bind(key)
            .execute(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!(
                    target: "SqliteKvStore::delete",
                    "Failed to delete key_value sqlite data: {e}",
                );
                KvStoreError::DeleteFailed
            })
    }
}

#[cfg(test)]
mod test {
    use super::SqliteKvStore;
    use crate::kvstore::KvStore;
    use sqlx::SqlitePool;
    use std::time::Duration;

    async fn connect() -> SqliteKvStore {
        let _ = env_logger::builder()
            .filter(None, log::LevelFilter::max())
            .filter(Some("sqlx"), log::LevelFilter::Warn)
            .try_init();

        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();

        SqliteKvStore::new(pool)
    }

    #[tokio::test]
    async fn test_get_set() {
        let kv = connect().await;

        let value1 = vec![8; 1024];
        let value2 = vec![5; 1024];

        kv.set("key1".into(), value1.clone()).await.unwrap();
        kv.set("key2".into(), value2.clone()).await.unwrap();

        let value1_result = kv
            .get("key1".into())
            .await
            .unwrap()
            .expect("Failed to get key after set");

        let value2_result = kv
            .get("key2".into())
            .await
            .unwrap()
            .expect("Failed to get key after set");

        assert!(value1.iter().eq(value1_result.iter()));
        assert!(value2.iter().eq(value2_result.iter()));
    }

    #[tokio::test]
    async fn test_set_override() {
        let kv = connect().await;

        let value = vec![5; 1024];

        kv.set("key".into(), vec![8; 1024]).await.unwrap();
        kv.set("key".into(), value.clone()).await.unwrap();

        let value_result = kv
            .get("key".into())
            .await
            .unwrap()
            .expect("Failed to get key after set");

        assert!(value.iter().eq(value_result.iter()));
    }

    #[tokio::test]
    async fn test_get_set_ttl() {
        const TTL: Duration = Duration::from_secs(60);

        let kv = connect().await;

        let value1 = vec![8; 1024];
        let value2 = vec![5; 1024];

        kv.set_ttl("key1".into(), value1.clone(), TTL)
            .await
            .unwrap();
        kv.set_ttl("key2".into(), value2.clone(), TTL)
            .await
            .unwrap();

        let value1_result = kv
            .get_ttl("key1".into(), TTL)
            .await
            .unwrap()
            .expect("Failed to get key after set");

        let value2_result = kv
            .get_ttl("key2".into(), TTL)
            .await
            .unwrap()
            .expect("Failed to get key after set");

        assert!(value1.iter().eq(value1_result.iter()));
        assert!(value2.iter().eq(value2_result.iter()));
    }

    #[tokio::test]
    async fn test_ttl_invalidation() {
        const TTL: Duration = Duration::from_millis(500);

        let kv = connect().await;

        let value = vec![8; 128];

        kv.set_ttl("key".into(), value.clone(), TTL).await.unwrap();

        let value_result = kv
            .get("key".into())
            .await
            .unwrap()
            .expect("Failed to get key after set");

        assert!(value.iter().eq(value_result.iter()));

        tokio::time::sleep(TTL * 2).await;

        let value_result = kv.get("key".into()).await.unwrap();
        assert_eq!(value_result, None);
    }

    #[tokio::test]
    async fn test_delete() {
        let kv = connect().await;

        kv.set("key".into(), Vec::new()).await.unwrap();
        kv.delete("key".into()).await.unwrap();

        let value_result = kv.get("key".into()).await.unwrap();
        assert_eq!(value_result, None);
    }
}
