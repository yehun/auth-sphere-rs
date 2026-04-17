use anyhow::{Result, anyhow};
use crate::{RedisService, RedisServiceOpt};
use crate::core::RedisServiceError;

impl RedisService {

    fn result<T>(result: Result<T, RedisServiceError>) -> Result<T> {
        match result {
            Ok(val) => Ok(val),
            Err(e) => Err(anyhow!(e))
        }
    }

    pub fn from_local() -> Result<Self> {
        let opt = RedisServiceOpt {
            host: "localhost".to_string(),
            port: 6379,
            db: Some(0),
            password: Some("root".to_string()),
            timeout: None,
        };
        Self::result(Self::from_opt(opt))
    }

    pub fn from_local_pika() -> Result<Self> {
        let opt = RedisServiceOpt {
            host: "localhost".to_string(),
            port: 9221,
            db: Some(0),
            password: Some("root".to_string()),
            timeout: None,
        };
        Self::result(Self::from_opt(opt))
    }

}
