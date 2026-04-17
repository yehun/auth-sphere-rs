use crate::core::error::RedisServiceError;
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// 分布式锁实现
pub struct RedisLock {
    client: Arc<Client>,
    name: String,
    lock_value: String,
}

impl RedisLock {
    pub(crate) fn new(client: Arc<Client>, name: String) -> Self {
        Self {
            client,
            name,
            lock_value: Uuid::new_v4().to_string(),
        }
    }

    async fn get_connection(&self) -> Result<MultiplexedConnection, RedisServiceError> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }

    async fn lock(&self) -> Result<(), RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let mut retries = 0;
        let max_retries = 10;
        let retry_delay = Duration::from_millis(100);

        loop {
            let acquired: bool = conn
                .set_nx::<&str, &String, bool>(&self.name, &self.lock_value)
                .await?;

            if acquired {
                // 设置过期时间防止死锁
                conn.expire::<&str, bool>(&self.name, 30i64).await?;
                return Ok(());
            }

            if retries >= max_retries {
                return Err(RedisServiceError::LockTimeout);
            }

            retries += 1;
            tokio::time::sleep(retry_delay).await;
        }
    }

    async fn try_lock(&self, wait_time: Duration, lease_time: Duration) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let start = std::time::Instant::now();
        let retry_delay = Duration::from_millis(100);

        while start.elapsed() < wait_time {
            let acquired: bool = conn
                .set_nx(&self.name, &self.lock_value)
                .await?;

            if acquired {
                conn.expire::<&str, bool>(&self.name, lease_time.as_secs() as i64).await?;
                return Ok(true);
            }

            tokio::time::sleep(retry_delay).await;
        }

        Ok(false)
    }
}


#[async_trait]
pub trait DistributedLock {
    async fn lock(&self) -> Result<(), RedisServiceError>;
    async fn try_lock(&self, wait_time: Duration, lease_time: Duration) -> Result<bool, RedisServiceError>;
    async fn unlock(&self) -> Result<(), RedisServiceError>;
    async fn is_locked(&self) -> Result<bool, RedisServiceError>;
}


#[async_trait]
impl DistributedLock for RedisLock {
    async fn lock(&self) -> Result<(), RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let mut retries = 0;
        let max_retries = 10;
        let retry_delay = Duration::from_millis(100);

        loop {
            let acquired: bool = conn
                .set_nx(&self.name, &self.lock_value)
                .await?;

            if acquired {
                // 设置过期时间防止死锁
                conn.expire::<&str, bool>(&self.name, 30i64).await?;
                return Ok(());
            }

            if retries >= max_retries {
                return Err(RedisServiceError::LockTimeout);
            }

            retries += 1;
            tokio::time::sleep(retry_delay).await;
        }
    }

    async fn try_lock(&self, wait_time: Duration, lease_time: Duration) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let start = std::time::Instant::now();
        let retry_delay = Duration::from_millis(100);

        while start.elapsed() < wait_time {
            let acquired: bool = conn
                .set_nx(&self.name, &self.lock_value)
                .await?;

            if acquired {
                conn.expire::<&str, bool>(&self.name, lease_time.as_secs() as i64).await?;
                return Ok(true);
            }

            tokio::time::sleep(retry_delay).await;
        }

        Ok(false)
    }

    async fn unlock(&self) -> Result<(), RedisServiceError> {
        let mut conn = self.get_connection().await?;

        // 使用 Lua 脚本确保只有锁的持有者才能释放锁
        let script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#;

        let result: i32 = redis::Script::new(script)
            .key(&self.name)
            .arg(&self.lock_value)
            .invoke_async(&mut conn)
            .await?;

        if result == 1 {
            Ok(())
        } else {
            Err(RedisServiceError::LockNotHeld)
        }
    }

    async fn is_locked(&self) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection().await?;
        let exists: bool = conn.exists(&self.name).await?;
        Ok(exists)
    }
}