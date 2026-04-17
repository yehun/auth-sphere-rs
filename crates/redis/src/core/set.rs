use crate::core::error::RedisServiceError;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

/// 分布式 Set 实现
pub struct RedisSet<T> {
    client: Arc<Client>,
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> RedisSet<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    pub(crate) fn new(client: Arc<Client>, name: String) -> Self {
        Self {
            client,
            name,
            _phantom: std::marker::PhantomData,
        }
    }

    async fn get_connection(&self) -> Result<MultiplexedConnection, RedisServiceError> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }

    fn serialize_value(&self, value: &T) -> Result<String, RedisServiceError> {
        serde_json::to_string(value).map_err(|e| RedisServiceError::SerializationError(e.to_string()))
    }
}

impl<T> RedisSet<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    pub async fn add(&self, value: &T) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let serialized = self.serialize_value(value)?;
        let added: bool = conn.sadd(&self.name, serialized).await?;
        Ok(added)
    }

    pub async fn remove(&self, value: &T) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let serialized = self.serialize_value(value)?;
        let removed: bool = conn.srem(&self.name, serialized).await?;
        Ok(removed)
    }

    pub async fn contains(&self, value: &T) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let serialized = self.serialize_value(value)?;
        let exists: bool = conn.sismember(&self.name, serialized).await?;
        Ok(exists)
    }

    pub async fn size(&self) -> Result<usize, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let size: usize = conn.scard(&self.name).await?;
        Ok(size)
    }

    pub async fn members(&self) -> Result<Vec<T>, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let members: Vec<String> = conn.smembers(&self.name).await?;
        let mut result = Vec::with_capacity(members.len());
        for member in members {
            let obj = serde_json::from_str(&member).map_err(|e| RedisServiceError::SerializationError(e.to_string()))?;
            result.push(obj);
        }
        Ok(result)
    }
}