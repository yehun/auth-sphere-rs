use std::time::Duration;
use anyhow::{bail, Result};
use tracing::debug;
use uuid::Uuid;
use auth_sphere_db::core::DatabasePool;
use auth_sphere_db::table::user::{User, UserId, UserRepository};
use auth_sphere_db::table::user_mfa::{UserMfa, UserMfaRepository, UserMfaUpdate};
use lib_mfa::{TotpConfig, TotpGenerator};
use lib_redis::RedisService;
use lib_sqlx::sqlx::Acquire;
use crate::server::model::response::auth::{LoginMfaResponse, LoginResponse};

pub struct UserMfaService {
    db: DatabasePool,
    redis: RedisService,
    mfa_config: TotpConfig,
}

impl UserMfaService {
    pub fn new(db: DatabasePool, redis: RedisService, mfa_config: TotpConfig) -> Self {
        Self { db, redis, mfa_config }
    }

    pub async fn login(&self, response: LoginResponse) -> Result<LoginMfaResponse> {
        // 用户开启了 MFA，生成临时令牌
        let temp_token = Uuid::new_v4().to_string();
        let user_info = response.user_info;
        let mfa_key = super::generate_mfa_key(&user_info.user_type, &temp_token);
        let user_data = serde_json::to_string(&user_info)?;
        self.redis.set_ex(&mfa_key, user_data.to_string(), Duration::from_secs(300)).await?;
        Ok(LoginMfaResponse {
            requires_2fa: true,
            token: temp_token,
            expired: 300
        })
    }

    pub async fn check(&self, user_id: UserId, username: &str, code: &str) -> Result<bool> {
        let mut conn = self.db.acquire().await?;
        let user_mfa = UserMfa::get_by_user_id(&mut conn, user_id).await?;
        let Some(user_mfa) = user_mfa else {
            bail!("user mfa not found")
        };
        let totp = TotpGenerator::new(
            self.mfa_config.clone(),
            &user_mfa.secret,
            username
        )?;
        let is_valid = totp.check_current(code)?;
        Ok(is_valid)
    }

    pub async fn generate(&self, user_id: UserId) -> Result<String> {
        let mut conn = self.db.acquire().await?;
        let user_mfa = UserMfa::get_by_user_id(&mut conn, user_id).await?;
        if let Some(user_mfa) = user_mfa {
            let row = UserMfa::delete(&mut conn, user_mfa.id).await?;
            debug!("user mfa delete row={row}");
        }
        let mfa_secret = TotpGenerator::generate_secret();
        let new_user_mfa = UserMfaUpdate {
            user_id,
            secret: mfa_secret.clone(),
        };
        let row = UserMfa::insert(&mut conn, new_user_mfa).await?;
        debug!("user mfa insert row={row}");
        Ok(mfa_secret)
    }

    pub async fn active(&self, user_id: UserId) -> Result<()> {
        let mut tx = self.db.begin().await?;
        let conn = tx.acquire().await?;
        let user_mfa = UserMfa::get_by_user_id(conn, user_id).await?;
        let Some(user_mfa) = user_mfa else {
            bail!("user mfa not found");
        };
        let mut row = 0;
        row += UserMfa::active(conn, user_mfa.id).await?;
        row += User::update_mfa(conn, user_id, true).await?;
        debug!("user mfa active row={row}");
        tx.commit().await?;
        Ok(())
    }

    pub async fn delete(&self, user_id: UserId) -> Result<()> {
        let mut tx = self.db.begin().await?;
        let conn = tx.acquire().await?;
        let user_mfa = UserMfa::get_by_user_id(conn, user_id).await?;
        let Some(user_mfa) = user_mfa else {
            bail!("user mfa not found");
        };
        let mut row = 0;
        row += UserMfa::delete(conn, user_mfa.id).await?;
        row += User::update_mfa(conn, user_id, false).await?;
        debug!("user mfa delete row={row}");
        tx.commit().await?;
        Ok(())
    }

}
