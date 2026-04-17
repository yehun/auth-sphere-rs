use anyhow::{bail, Result};
use auth_sphere_db::table::user::{User, UserInsert, UserKind, UserRepository, UserStatus};
use auth_sphere_db::table::user_password::{UserPassword, UserPasswordInsert, UserPasswordRepository};
use auth_sphere_db::table::user_email::{UserEmail, UserEmailInsert, UserEmailRepository};
use auth_sphere_db::table::user_phone::{UserPhone, UserPhoneInsert, UserPhoneRepository};
use auth_sphere_db::table::user_session::{UserDevice, UserSession, UserSessionInsert, UserSessionRepository};
use auth_sphere_db::table::user_verify_code::{UserVerifyCode, UserVerifyCodeInsert, UserVerifyCodeRepository, UserVerifySourceKind};
use auth_sphere_db::core::DatabasePool;
use serde::{Deserialize, Serialize};
use tracing::{debug};
use uuid::Uuid;
use std::time::Duration;
use lib_redis::RedisService;
use lib_sqlx::sqlx::Acquire;
use crate::server::model::response::auth::{LoginResponse, UserInfo, OtpSendResponse, RegisterResponse, LoginMfaResponse};
use crate::server::service::LoginType;

/// 会话信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub user_id: u64,
    pub username: String,
    pub nickname: String,
    pub user_type: UserKind,
    pub device: UserDevice,
    pub token: String,
}

impl TryFrom<String> for Session {
    type Error = serde_json::Error;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        serde_json::from_str::<Self>(&value)
    }
}

impl Session {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

pub struct AuthService {
    db: DatabasePool,
    redis: RedisService,
    // token_store: TokenStore,
    // otp_store: Arc<RwLock<HashMap<String, String>>>, // key: contact, value: otp_code
}

impl AuthService {
    pub fn new(db: DatabasePool, redis: RedisService) -> Self {
        Self {
            db,
            redis,
            // token_store: Arc::new(RwLock::new(HashMap::new())),
            // otp_store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 发送 OTP
    pub async fn send_otp(
        &self,
        email: Option<String>,
        phone: Option<String>,
        user_device: UserDevice,
        user_type: UserKind
    ) -> Result<OtpSendResponse> {
        let contact = email.or(phone).ok_or_else(|| anyhow::anyhow!("邮箱或手机号必须提供一个"))?;
        let mut conn = self.db.acquire().await?;
        let (user_id, source_kind) = match super::login_type_from_str(&contact) {
            LoginType::Email => {
                let user_id = UserEmail::get_by_email(&mut conn, &contact).await?
                    .map(|x| x.user_id);
                (user_id, UserVerifySourceKind::Email)
            },
            LoginType::Phone => {
                let user_id = UserPhone::get_by_phone(&mut conn, &contact).await?.map(|x| x.user_id);
                (user_id, UserVerifySourceKind::Phone)
            },
            LoginType::Username => bail!("未找到用户")
        };
        let Some(user_id) = user_id else {
            bail!("该邮箱或手机号未注册");
        };

        let Some(user) = User::get_by_id(&mut conn, user_id).await? else {
            bail!("未找到用户");
        };

        let user_kind: UserKind = user.kind.into();
        if user_kind != user_type {
            bail!("未找到用户");
        }

        // 生成 6 位随机验证码
        let otp_code = format!("{:06}", rand::random::<u32>() % 1000000);

        let user_verify_code = UserVerifyCodeInsert {
            user_id,
            source_kind: source_kind as u8,
            source: contact.clone(),
            code: otp_code.clone(),
        };
        if UserVerifyCode::insert(&mut conn, user_verify_code).await? <= 0 {
            bail!("保存验证码错误")
        }

        debug!("Platform OTP code for {}: {} (开发环境，生产环境应通过短信/邮件发送)", contact, otp_code);
        let otp_key = super::generate_otp_key(&user_kind, &user_device, &contact);
        self.redis.set_ex(&otp_key, otp_code, Duration::from_secs(60)).await?;

        Ok(OtpSendResponse {
            success: true,
            message: "验证码已发送（开发环境请查看日志）".to_string(),
            expire_in: 300, // 5 分钟有效期
        })
    }

    /// 验证密码
    async fn verify_password(&self, user_id: u64, password: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;

        let user_password = UserPassword::get_by_user_id(&mut conn, user_id).await?;
        let user_password = match user_password {
            Some(up) => up,
            None => bail!("用户密码未设置"),
        };

        let digest_password = Self::hash_password(password)?;
        if user_password.password != digest_password {
            bail!("用户名或密码错误");
        }

        Ok(())
    }

    fn hash_password(password: &str) -> Result<String> {
        let digest = md5::compute(password.as_bytes());
        Ok(format!("{:x}", digest))
    }

    pub async fn create_session(&self, user: &User, device: UserDevice) -> Result<Session> {
        let token = Uuid::new_v4().to_string();

        let user_kind: UserKind = user.kind.into();
        let session = Session {
            user_id: user.id,
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            user_type: user_kind.clone(),
            device: device.clone(),
            token: token.clone(),
        };

        // 检查并删除旧的会话
        let mut conn = self.db.acquire().await?;
        if let Some(user_session) = UserSession::get_by_user_id(&mut conn, user.id, device.clone()).await? {
            let _row = UserSession::delete(&mut conn, user_session.id).await?;
            let user_login_key = super::generate_login_key(&user_kind, &user_session.token);
            self.redis.delete(&user_login_key).await?;
        }

        let login_key = super::generate_login_key(&user_kind, &token);
        // 存储会话到数据库
        let user_session = UserSessionInsert {
            user_id: user.id,
            device: device as u8,
            token: token.clone(),
        };
        UserSession::insert(&mut conn, user_session).await?;

        let session_json = session.to_json()?;
        self.redis.set(&login_key, session_json).await?;
        Ok(session)
    }

    pub fn session_to_login_response(&self, session: Session, user: &User) -> LoginResponse {
        LoginResponse {
            token: session.token.clone(),
            expires_in: 3600, // 1 小时
            user_info: UserInfo {
                id: session.user_id,
                username: session.username,
                nickname: session.nickname,
                user_type: session.user_type,
                is_mfa: user.is_mfa,
                is_passkey: user.is_passkey,
            },
        }
    }

    pub async fn login_with_password(
        &self,
        username: &str,
        password: &str,
        device: UserDevice,
        expected_kind: UserKind,
    ) -> Result<(LoginResponse, bool)> {
        debug!("Login with password for username: {}, type: {:?}", username, expected_kind);

        let mut conn = self.db.acquire().await?;

        // 查询用户
        let user = User::get_by_username(&mut conn, username).await?;
        let user = match user {
            Some(u) => u,
            None => bail!("用户名或密码错误"),
        };

        // 验证用户类型
        let user_kind: UserKind = user.kind.into();
        if user_kind != expected_kind {
            bail!("该用户不是{:?}账户", expected_kind);
        }

        // 验证状态
        let status: UserStatus = user.status().into();
        if status != UserStatus::Normal {
            bail!("账户状态异常: {:?}", status);
        }
        // 验证密码
        self.verify_password(user.id, password).await?;

        // 用户未开启 MFA，直接生成会话
        let session = self.create_session(&user, device).await?;

        Ok((self.session_to_login_response(session, &user), user.is_mfa))
    }

    pub async fn login_with_code(
        &self,
        contact: &str,
        otp_code: &str,
        device: UserDevice,
        expected_kind: UserKind,
    ) -> Result<LoginResponse> {
        debug!("Login with code for contact: {}", contact);

        let mut conn = self.db.acquire().await?;

        let user_id = match super::login_type_from_str(&contact) {
            LoginType::Email => UserEmail::get_by_email(&mut conn, &contact).await?.map(|x| x.user_id),
            LoginType::Phone => UserPhone::get_by_phone(&mut conn, &contact).await?.map(|x| x.user_id),
            LoginType::Username => bail!("请使用邮箱或手机号登录")
        };
        let Some(user_id) = user_id else {
            bail!("该邮箱或手机号未注册");
        };

        // 验证 OTP

        let otp_key = super::generate_otp_key(&expected_kind, &device, &contact);
        let Ok(code) = self.redis.get(&otp_key).await else {
            bail!("验证码已过期")
        };
        if code != otp_code {
            bail!("验证码错误")
        }
        self.redis.delete(&otp_key).await?;
        let _row = UserVerifyCode::verify(&mut conn, user_id, &contact).await?;

        let user = User::get_by_id(&mut conn, user_id).await?;
        let Some(user) = user else {
            bail!("未找到用户");
        };

        // 验证用户类型
        let user_kind: UserKind = (user.kind).into();
        if user_kind != expected_kind {
            bail!("该用户不是{:?}账户", expected_kind);
        }

        // 生成会话
        let session = self.create_session(&user, device).await?;

        Ok(self.session_to_login_response(session, &user))
    }

    /// 通用登出方法
    pub async fn logout(&self, user_kind: UserKind, access_token: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        if let Some(user_session) = UserSession::get_by_token(&mut conn, access_token).await? {
            let _row = UserSession::delete(&mut conn, user_session.id).await?;
        }
        let login_key = super::generate_login_key(&user_kind, access_token);
        self.redis.delete(&login_key).await?;
        debug!("User logged out");
        Ok(())
    }

    /// 通用注册方法
    pub async fn register(
        &self,
        nickname: &str,
        username: &str,
        password: &str,
        kind: UserKind,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<RegisterResponse> {
        debug!("Register attempt for username: {}, type: {:?}", username, kind);

        let mut tx = self.db.begin().await?;
        let conn = tx.acquire().await?;

        if User::get_by_username(conn, username).await?.is_some() {
            bail!("用户名已存在");
        }

        // 创建用户
        let user = UserInsert {
            kind,
            nickname: nickname.to_string(),
            username: username.to_string(),
            status: UserStatus::Normal,
        };

        let (_, user_id) = User::insert(conn, user).await?;

        // 存储密码
        let user_password = UserPasswordInsert {
            user_id,
            password: Self::hash_password(password)?,
        };
        UserPassword::insert(conn, user_password).await?;

        if let Some(email) = email {
            debug!("Email registered: {}", email);
            let user_email = UserEmailInsert { user_id, email };
            let _row = UserEmail::insert(conn, user_email).await?;
        }
        if let Some(phone) = phone {
            debug!("Phone registered: {}", phone);
            let user_phone = UserPhoneInsert { user_id, phone };
            let _row = UserPhone::insert(conn, user_phone).await?;
        }
        tx.commit().await?;
        Ok(RegisterResponse {
            user_id,
            username: username.to_string(),
            message: "注册成功".to_string(),
        })
    }
}
