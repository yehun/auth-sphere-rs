use std::sync::Arc;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use crate::RedisServiceError;

pub struct RedisConfig {
    client: Arc<Client>,
}

impl RedisConfig {
    pub(crate) fn new(client: Arc<Client>) -> Self {
        Self {
            client
        }
    }

    async fn get_connection(&self) -> Result<MultiplexedConnection, RedisServiceError> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }

    pub async fn keys(&self, pattern: &str) -> Result<Vec<String>, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let result = conn.keys::<&str, Vec<String>>(pattern).await?;
        Ok(result)
    }

}
