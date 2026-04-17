use tracing::{debug, error};
use webauthn_rs::prelude::*;
use webauthn_rs::Webauthn;
use auth_sphere_db::core::DatabasePool;
use auth_sphere_db::table::user::{User, UserId, UserKind, UserRepository};
use lib_redis::RedisService;
use serde_json;
use std::time::Duration;
use auth_sphere_db::table::user_passkey::{UserPassKey, UserPassKeyInsert, UserPassKeyRepository};
use lib_sqlx::sqlx::Acquire;

pub struct UserPassKeyService {
    db: DatabasePool,
    redis: RedisService,
    webauthn: Webauthn,
}

impl UserPassKeyService {
    pub fn new(db: DatabasePool, redis: RedisService, webauthn: Webauthn) -> Self {
        Self { db, redis, webauthn }
    }

    pub async fn deactived(&self, user_id: UserId) -> Result<(), String> {
        let mut tx = self.db.begin().await.map_err(|e| {
            error!("Failed to database transaction: {:?}", e);
            "获取数据库实务失败".to_string()
        })?;
        let conn = tx.acquire().await.map_err(|e| {
            error!("Failed to acquire database connection: {:?}", e);
            "获取数据库连接失败".to_string()
        })?;
        let mut row = 0;
        row += User::update_passkey(conn, user_id, false).await.map_err(|e| {
            error!("Failed to update user: {:?}", e);
            "更新用户Passkey开关失败".to_string()
        })?;
        row += UserPassKey::delete_by_user_id(conn, user_id).await.map_err(|e| {
            error!("Failed to writer data: {:?}", e);
            "写入数据失败".to_string()
        })?;
        debug!("deactive row={row}");
        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction: {:?}", e);
            "提交事务失败".to_string()
        })?;
        Ok(())
    }


    /// 开始 Passkey 注册流程
    pub async fn register_begin(
        &self,
        username: &str,
        user_id: UserId,
        display_name: &str,
    ) -> Result<CreationChallengeResponse, String> {
        debug!("Starting passkey registration for user: {}", username);
        
        let (creation_challenge, passkey_reg) = self.webauthn
            .start_passkey_registration(
                Uuid::from_u64_pair(0, user_id as u64),
                username,
                display_name,
                None,
            )
            .map_err(|e| {
                error!("Failed to start passkey registration: {:?}", e);
                format!("启动 Passkey 注册失败: {:?}", e)
            })?;

        // 将 creation_challenge 和 passkey_reg 都存储到 Redis
        let challenge_key = format!("passkey::register::challenge::{}", user_id);
        let challenge_json = serde_json::to_string(&creation_challenge).map_err(|e| {
            error!("Failed to serialize challenge: {:?}", e);
            format!("序列化挑战失败: {:?}", e)
        })?;
        
        // 使用 serde_json 序列化 PasskeyRegistration
        let reg_json = serde_json::to_string(&passkey_reg).map_err(|e| {
            error!("Failed to serialize passkey_reg: {:?}", e);
            format!("序列化注册数据失败: {:?}", e)
        })?;
        
        self.redis.set_ex(&challenge_key, challenge_json.clone(), Duration::from_secs(300)).await.map_err(|e| {
            error!("Failed to store challenge in Redis: {:?}", e);
            format!("存储挑战失败: {:?}", e)
        })?;
        
        // 存储序列化的 passkey_reg
        let reg_key = format!("passkey::register::reg::{}", user_id);
        self.redis.set_ex(&reg_key, reg_json, Duration::from_secs(300)).await.map_err(|e| {
            error!("Failed to store passkey_reg in Redis: {:?}", e);
            format!("存储注册数据失败: {:?}", e)
        })?;

        Ok(creation_challenge)
    }

    /// 完成 Passkey 注册流程
    pub async fn register_complete(
        &self,
        username: &str,
        user_id: UserId,
        credential: serde_json::Value,
    ) -> Result<(), String> {
        debug!("Completing passkey registration for user: {}", username);

        // 从 Redis 获取 passkey_reg
        let reg_key = format!("passkey::register::reg::{}", user_id);
        let reg_json = self.redis.get(&reg_key).await.map_err(|e| {
            error!("Failed to get passkey_reg from Redis: {:?}", e);
            "获取注册数据失败".to_string()
        })?;

        // let reg_json = reg_json_opt.ok_or_else(|| "注册数据不存在或已过期".to_string())?;
        
        // serde_json 反序列化
        let passkey_reg: PasskeyRegistration = serde_json::from_str(&reg_json).map_err(|e| {
            error!("Failed to deserialize passkey_reg: {:?}", e);
            "反序列化注册数据失败".to_string()
        })?;

        // 解析前端传来的 credential
        let reg_response: RegisterPublicKeyCredential = serde_json::from_value(credential).map_err(|e| {
            error!("Failed to parse credential: {:?}", e);
            "解析凭证失败".to_string()
        })?;

        // 验证注册响应
        let passkey = self.webauthn
            .finish_passkey_registration(&reg_response, &passkey_reg)
            .map_err(|e| {
                error!("Failed to finish passkey registration: {:?}", e);
                format!("验证 Passkey 注册失败: {:?}", e)
            })?;

        // 保存 Passkey 到数据库
        // cred_id() 返回 HumanBinaryData，我们需要获取其内部的 bytes
        let credential_id_bytes = passkey.cred_id().as_ref();
        let credential_id = hex::encode(credential_id_bytes);
        
        let public_key = serde_json::to_string(&passkey).map_err(|e| {
            error!("Failed to serialize public key: {:?}", e);
            "序列化公钥失败".to_string()
        })?;

        let insert = UserPassKeyInsert {
            user_id,
            credential_id: credential_id.clone(),
            public_key,
            sign_count: 0,
        };
        let mut tx = self.db.begin().await.map_err(|e| {
            error!("Failed to database transaction: {:?}", e);
            "获取数据库实务失败".to_string()
        })?;
        let conn = tx.acquire().await.map_err(|e| {
            error!("Failed to acquire database connection: {:?}", e);
            "获取数据库连接失败".to_string()
        })?;
        let mut row = 0;
        row += User::update_passkey(conn, user_id, true).await.map_err(|e| {
            error!("Failed to update user: {:?}", e);
            "更新用户Passkey开关失败".to_string()
        })?;
        row += UserPassKey::insert(conn, insert).await.map_err(|e| {
            error!("Failed to writer data: {:?}", e);
            "写入数据失败".to_string()
        })?;
        debug!("register passkey row={row}");
        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction: {:?}", e);
            "提交事务失败".to_string()
        })?;
        
        debug!("Passkey registered for user {} with credential_id: {}, db_row={}", username, credential_id, row);

        // 删除 Redis 中的临时数据
        let challenge_key = format!("passkey::register::challenge::{}", user_id);
        self.redis.delete(&challenge_key).await.map_err(|e| {
            error!("Failed to delete challenge: {:?}", e);
            "删除挑战失败".to_string()
        })?;
        
        self.redis.delete(&reg_key).await.map_err(|e| {
            error!("Failed to delete passkey_reg: {:?}", e);
            "删除注册数据失败".to_string()
        })?;

        debug!("Passkey registration completed successfully for user: {}", username);
        Ok(())
    }

    /// 开始 Passkey 登录流程
    pub async fn login_begin(
        &self,
        username: &str,
        user_id: UserId,
    ) -> Result<RequestChallengeResponse, String> {
        debug!("Starting passkey login for user: {}", username);

        // 获取用户的所有 active passkeys
        let mut conn = self.db.acquire().await.map_err(|e| {
            error!("Failed to acquire database connection: {:?}", e);
            "获取数据库连接失败".to_string()
        })?;

        let passkeys = UserPassKey::get_by_user_id(&mut conn, user_id).await.map_err(|e| {
            error!("Failed to get passkeys: {:?}", e);
            "获取 Passkey 失败".to_string()
        })?;

        let passkey = passkeys.ok_or("用户未注册 Passkey")?;

        // 解析 stored credential
        let stored_credential: Passkey = serde_json::from_str(&passkey.public_key).map_err(|e| {
            error!("Failed to parse stored credential: {:?}", e);
            "解析存储的凭证失败".to_string()
        })?;

        // 开始认证挑战
        let (challenge, passkey_auth) = self.webauthn
            .start_passkey_authentication(&[stored_credential])
            .map_err(|e| {
                error!("Failed to start passkey authentication: {:?}", e);
                format!("启动 Passkey 认证失败: {:?}", e)
            })?;

        // 将 challenge 和 passkey_auth 都存储到 Redis
        let challenge_key = format!("passkey::login::challenge::{}", user_id);
        let challenge_json = serde_json::to_string(&challenge).map_err(|e| {
            error!("Failed to serialize challenge: {:?}", e);
            format!("序列化挑战失败: {:?}", e)
        })?;
        
        // 序列化 passkey_auth
        let auth_json = serde_json::to_string(&passkey_auth).map_err(|e| {
            error!("Failed to serialize passkey_auth: {:?}", e);
            format!("序列化认证数据失败: {:?}", e)
        })?;

        self.redis.set_ex(&challenge_key, challenge_json.clone(), Duration::from_secs(300)).await.map_err(|e| {
            error!("Failed to store challenge in Redis: {:?}", e);
            format!("存储挑战失败: {:?}", e)
        })?;
        
        // 存储 passkey_auth
        let auth_key = format!("passkey::login::auth::{}", user_id);
        self.redis.set_ex(&auth_key, auth_json, Duration::from_secs(300)).await.map_err(|e| {
            error!("Failed to store passkey_auth in Redis: {:?}", e);
            format!("存储认证数据失败: {:?}", e)
        })?;

        Ok(challenge)
    }

    /// 完成 Passkey 登录流程
    pub async fn login_complete(
        &self,
        username: &str,
        user_id: UserId,
        credential: serde_json::Value,
    ) -> Result<(), String> {
        debug!("Completing passkey login for user: {}", username);

        // 从 Redis 获取 challenge 和 passkey_auth
        let challenge_key = format!("passkey::login::challenge::{}", user_id);
        // let challenge_json = self.redis.get(&challenge_key).await.map_err(|e| {
        //     error!("Failed to get challenge from Redis: {:?}", e);
        //     "获取挑战失败".to_string()
        // })?;
        //
        // let challenge: RequestChallengeResponse = serde_json::from_str(&challenge_json).map_err(|e| {
        //     error!("Failed to deserialize challenge: {:?}", e);
        //     "反序列化挑战失败".to_string()
        // })?;

        // 获取 passkey_auth
        let auth_key = format!("passkey::login::auth::{}", user_id);
        let auth_json = self.redis.get(&auth_key).await.map_err(|e| {
            error!("Failed to get passkey_auth from Redis: {:?}", e);
            "获取认证数据失败".to_string()
        })?;
        
        let passkey_auth: PasskeyAuthentication = serde_json::from_str(&auth_json).map_err(|e| {
            error!("Failed to deserialize passkey_auth: {:?}", e);
            "反序列化认证数据失败".to_string()
        })?;

        // 解析前端传来的 credential
        let auth_response: PublicKeyCredential = serde_json::from_value(credential).map_err(|e| {
            error!("Failed to parse credential: {:?}", e);
            "解析凭证失败".to_string()
        })?;

        let mut conn = self.db.acquire().await.map_err(|e| {
            error!("Failed to acquire database connection: {:?}", e);
            "获取数据库连接失败".to_string()
        })?;
        let passkey_record = UserPassKey::get_by_user_id(&mut conn, user_id).await.map_err(|e| {
            error!("Failed to get passkey: {:?}", e);
            "获取用户passkey失败".to_string()
        })?;
        let Some(passkey_record) = passkey_record else {
            return Err("用户未注册 Passkey".to_string());
        };

        // 验证认证响应
        let auth_result = self.webauthn
            .finish_passkey_authentication(&auth_response, &passkey_auth)
            .map_err(|e| {
                error!("Failed to finish passkey authentication: {:?}", e);
                format!("验证 Passkey 认证失败: {:?}", e)
            })?;

        // 更新 sign_count
        let new_sign_count = auth_result.counter().cast_signed() as u32;
        UserPassKey::update_sign_count(&mut conn, passkey_record.id, new_sign_count).await.map_err(|e| {
            error!("Failed to update sign count: {:?}", e);
            "更新签名计数失败".to_string()
        })?;

        debug!("Passkey authentication successful for user: {}, new sign count: {}", username, new_sign_count);

        // 删除 Redis 中的临时数据
        self.redis.delete(&challenge_key).await.map_err(|e| {
            error!("Failed to delete challenge: {:?}", e);
            "删除挑战失败".to_string()
        })?;
        
        self.redis.delete(&auth_key).await.map_err(|e| {
            error!("Failed to delete passkey_auth: {:?}", e);
            "删除认证数据失败".to_string()
        })?;

        debug!("Passkey login completed successfully for user: {}", username);
        Ok(())
    }
}
