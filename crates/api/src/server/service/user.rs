use auth_sphere_db::core::DatabasePool;
use auth_sphere_db::Repository;
use auth_sphere_db::table::user::{User, UserId, UserKind, UserRepository};
use lib_redis::RedisService;
use crate::server::model::response::auth::UserInfo;
use crate::server::service::auth::Session;

pub struct UserService {
    db: DatabasePool,
    redis: RedisService,
}

impl UserService {
    pub fn new(db: DatabasePool, redis: RedisService) -> Self {
        Self {
            db,
            redis,
        }
    }

    pub async fn get_user(&self, user_kind: UserKind, user_id: UserId) -> anyhow::Result<User> {
        let mut conn = self.db.acquire().await?;
        let Some(user) = User::get(&mut conn, user_id).await? else {
            return Err(anyhow::anyhow!("用户不存在"));
        };
        if user.kind != user_kind as u8 {
            return Err(anyhow::anyhow!("用户类型不匹配"));
        }
        Ok(user)
    }

    /// 根据用户名获取用户
    pub async fn get_by_username(&self, user_kind: UserKind, username: &str) -> anyhow::Result<Option<User>> {
        let mut conn = self.db.acquire().await?;
        let user = User::get_by_username(&mut conn, username).await?;
        // 验证用户类型是否匹配
        if let Some(ref user) = user {
            if user.kind != user_kind as u8 {
                return Err(anyhow::anyhow!("用户类型不匹配"));
            }
        }
        Ok(user)
    }

    /// 获取当前用户信息
    pub async fn current_user(&self, user_kind: &UserKind, token: &str) -> anyhow::Result<UserInfo> {
        let login_key = super::generate_login_key(user_kind, token);
        let value = self.redis.get(&login_key).await?;
        let session = Session::try_from(value)?;
        let mut conn = self.db.acquire().await?;
        let Some(user) = User::get(&mut conn, session.user_id).await? else {
            return Err(anyhow::anyhow!("用户不存在"));
        };
        Ok(UserInfo {
            id: user.id,
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            user_type: user.kind.into(),
            is_mfa: user.is_mfa,
            is_passkey: user.is_passkey,
        })
    }


}