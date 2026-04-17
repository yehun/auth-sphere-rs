use webauthn_rs::Webauthn;
use auth_sphere_db::core::DatabasePool;
use lib_redis::RedisService;

pub struct UserPassKeyService {
    db: DatabasePool,
    redis: RedisService,
    webauthn: Webauthn,
}

impl UserPassKeyService {
    pub fn new(db: DatabasePool, redis: RedisService, webauthn: Webauthn) -> Self {
        Self { db, redis, webauthn }
    }
}
