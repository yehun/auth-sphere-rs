use thiserror::Error;
#[derive(Error, Debug)]
pub enum RedisServiceError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Lock acquisition timeout")]
    LockTimeout,
    #[error("Lock not held by this client")]
    LockNotHeld,
}
