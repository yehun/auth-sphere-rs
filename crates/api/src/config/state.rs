use std::sync::Arc;
use lib_mfa::TotpConfig;
use lib_redis::RedisService;
use crate::server::service::{AuthService, UserService, UserMfaService, UserPassKeyService};

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisService,
    pub mfa_config: TotpConfig,
    pub mfa_service: Arc<UserMfaService>,
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
    pub passkey_service: Arc<UserPassKeyService>,

}



