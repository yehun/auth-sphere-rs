use crate::core::error::RedisServiceError;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

/// 分布式 Map 实现
pub struct RedisMap<K, V> {
    client: Arc<Client>,
    name: String,
    _phantom_k: std::marker::PhantomData<K>,
    _phantom_v: std::marker::PhantomData<V>,
}

impl<K, V> RedisMap<K, V>
where
    K: Serialize + DeserializeOwned + Send + Sync,
    V: Serialize + DeserializeOwned + Send + Sync,
{
    pub(crate) fn new(client: Arc<Client>, name: String) -> Self {
        Self {
            client,
            name,
            _phantom_k: std::marker::PhantomData,
            _phantom_v: std::marker::PhantomData,
        }
    }

    async fn get_connection(&self) -> Result<MultiplexedConnection, RedisServiceError> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }

    fn make_field_key(&self, field: &K) -> Result<String, RedisServiceError> {
        let serialized = serde_json::to_string(field).map_err(|e| RedisServiceError::SerializationError(e.to_string()))?;
        Ok(format!("{}:{}", self.name, serialized))
    }
}

impl<K, V> RedisMap<K, V>
where
    K: Serialize + DeserializeOwned + Send + Sync,
    V: Serialize + DeserializeOwned + Send + Sync,
{
    pub async fn put(&self, key: &K, value: &V) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let field_key = self.make_field_key(key)?;
        let serialized = serde_json::to_string(value)
            .map_err(|e| RedisServiceError::SerializationError(e.to_string()))?;
        Ok(conn.hset::<&str, String, String, bool>(&self.name, field_key, serialized).await?)
    }

    pub async fn get(&self, key: &K) -> Result<Option<V>, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let field_key = self.make_field_key(key)?;
        let value: Option<String> = conn.hget(&self.name, field_key).await?;
        match value {
            Some(v) => {
                let obj = serde_json::from_str(&v)
                    .map_err(|e| RedisServiceError::SerializationError(e.to_string()))?;
                Ok(Some(obj))
            }
            None => Ok(None),
        }
    }

    pub async fn remove(&self, key: &K) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let field_key = self.make_field_key(key)?;
        let deleted: i32 = conn.hdel(&self.name, field_key).await?;
        Ok(deleted > 0)
    }

    pub async fn contains_key(&self, key: &K) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let field_key = self.make_field_key(key)?;
        let exists: bool = conn.hexists(&self.name, field_key).await?;
        Ok(exists)
    }

    pub async fn size(&self) -> Result<usize, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let size: usize = conn.hlen(&self.name).await?;
        Ok(size)
    }
}