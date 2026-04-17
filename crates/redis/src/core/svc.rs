use crate::core::builder::RedisServiceOpt;
use crate::core::error::RedisServiceError;
use crate::core::lock::RedisLock;
use crate::core::map::RedisMap;
use crate::core::set::RedisSet;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client, ConnectionAddr, ConnectionInfo, ConnectionLike, IntoConnectionInfo, RedisConnectionInfo};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;
use crate::core::config::RedisConfig;

/// 分布式锁接口

/// Redis 服务主结构
#[derive(Clone)]
pub struct RedisService {
    client: Arc<Client>,
}

impl RedisService {

    pub fn from_opt(opt: RedisServiceOpt) -> Result<Self, RedisServiceError> {
        let conn = ConnectionInfo {
            addr: ConnectionAddr::Tcp(opt.host, opt.port),
            redis: RedisConnectionInfo {
                db: opt.db.unwrap_or(0) as i64,
                password: opt.password,
                username: None,
                protocol: Default::default(),
            },
        };
        Self::new(conn)
    }

    pub fn from_uri(uri: String) -> Result<Self, RedisServiceError> {
        Self::new(uri)
    }

    pub fn new<T: IntoConnectionInfo>(param: T) -> Result<Self, RedisServiceError> {
        let client = Client::open(param)?;
        Ok(Self {
            client: Arc::new(client)
        })
    }

    pub async fn get_connection(&self) -> Result<MultiplexedConnection, RedisServiceError> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }

    pub async fn is_open(&self) -> Result<bool, RedisServiceError> {
        Ok(self.client.get_connection()?.is_open())
    }

    pub fn get_config(&self) -> RedisConfig {
        RedisConfig::new(self.client.clone())
    }

    pub async fn get(&self, key: &str) -> Result<String, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        Ok(conn.get::<&str, String>(key).await?)
    }

    pub async fn set(&self, key: &str, value: String) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        Ok(conn.set::<&str, String, bool>(key, value).await?)
    }

    pub async fn set_ex(&self, key: &str, value: String, ttl: Duration) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        Ok(conn.set_ex::<&str, String, bool>(key, value, ttl.as_secs())
            .await?)
    }

    pub async fn delete(&self, key: &str) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let deleted: i32 = conn.del(key).await?;
        Ok(deleted > 0)
    }

    pub async fn deletes(&self, keys: &[&String; 2]) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let deleted: i32 = conn.del(keys).await?;
        Ok(deleted > 0)
    }

    pub async fn exists(&self, key: &str) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let exists: bool = conn.expire(key, ttl.as_secs() as i64).await?;
        Ok(exists)
    }

    pub async fn ttl(&self, key: &str) -> Result<Duration, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let ttl: i64 = conn.ttl(key).await?;
        Ok(Duration::from_secs(ttl as u64))
    }

    /// 获取分布式锁
    pub fn get_lock(&self, name: &str) -> RedisLock {
        RedisLock::new(self.client.clone(), name.to_string())
    }

    /// 获取分布式 Map
    pub fn get_map<K, V>(&self, name: &str) -> RedisMap<K, V>
    where
        K: Serialize + DeserializeOwned + Send + Sync + 'static,
        V: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        RedisMap::new(self.client.clone(), name.to_string())
    }

    /// 获取分布式 Set
    pub fn get_set<T>(&self, name: &str) -> RedisSet<T>
    where
        T: Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        RedisSet::new(self.client.clone(), name.to_string())
    }

    // #[cfg(feature = "connection-manager")]
    // pub async fn get_connection_manager(&self) -> anyhow::Result<aio::ConnectionManager> {
    //     Ok(aio::ConnectionManager::new(self.client.clone()).await?)
    // }
}

