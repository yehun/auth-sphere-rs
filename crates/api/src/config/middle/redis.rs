use crate::config::application;
use lib_redis::{RedisService, RedisServiceOpt};

pub fn init() -> RedisService {
    let config = application::get();
    let opt = RedisServiceOpt {
        host: config.redis.host,
        port: config.redis.port.unwrap_or(6379),
        db: config.redis.db,
        password: config.redis.auth,
        timeout: config.redis.timeout,
    };
    RedisService::from_opt(opt).expect("redis init error")
}