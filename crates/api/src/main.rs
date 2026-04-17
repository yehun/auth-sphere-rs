mod config;
mod server;
mod utils;

use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;
use clap::Parser;
use tracing::debug;
use crate::config::middle;
use crate::server::service::{AuthService, UserMfaService, UserPassKeyService, UserService};

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[arg(short = 'c', long = "config", default_value = "/home/yehun/auth-sphere.toml")]
    config: PathBuf,
}

#[actix_web::main]
async fn main() -> Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=debug");
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    let command = Command::parse();
    debug!("load config: {}", command.config.display());
    let _guard = config::init(command.config);
    
    let db = middle::db::init().await;
    let redis = middle::redis::init().await;
    let webauthn = middle::webauthn::init();
    let mfa_config = middle::mfa::init();
    let mfa_service = Arc::new(UserMfaService::new(
        db.clone(),
        redis.clone(),
        mfa_config.clone()
    ));
    let auth_service = Arc::new(AuthService::new(db.clone(), redis.clone()));
    let user_service = Arc::new(UserService::new(db.clone(), redis.clone()));
    let passkey_service = Arc::new(UserPassKeyService::new(
        db,
        redis.clone(),
        webauthn
    ));

    let web_server = server::init(config::AppState {
        redis,
        mfa_config,
        mfa_service,
        auth_service,
        user_service,
        passkey_service
    }).await?;
    Ok(web_server.await?)
}
