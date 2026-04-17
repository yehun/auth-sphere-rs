use crate::config::application;
use lib_redis::{RedisService, RedisServiceOpt};

pub async fn init() -> RedisService {
    let config = application::get();
    let opt = RedisServiceOpt {
        host: config.redis.host,
        port: config.redis.port.unwrap_or(6379),
        db: config.redis.db,
        password: config.redis.auth,
        timeout: config.redis.timeout,
    };
    let client = RedisService::from_opt(opt).expect("redis init error");
    if !client.is_open().await.expect("redis connect error") {
        panic!("redis connect error");
    }
    client
}