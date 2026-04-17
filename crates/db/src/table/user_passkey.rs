use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use lib_sqlx::sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserPassKey {
    pub id: u64,
    pub user_id: UserId,
    pub credential_id: String,
    pub public_key: String,
    pub sign_count: u32,
    pub active: bool,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserPassKeyInsert {
    pub user_id: UserId,
    pub credential_id: String,
    pub public_key: String,
    pub sign_count: u32,
    pub active: bool,
}

#[allow(async_fn_in_trait)]
pub trait UserPassKeyRepository<E: DatabaseExecutor>: Repository<E, UserPassKey, u64> {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserPassKey>, Error>;
    async fn get_by_credential_id(executor: &mut E, credential_id: &str) -> Result<Option<UserPassKey>, Error>;
    async fn insert(executor: &mut E, data: UserPassKeyInsert) -> Result<u64, Error>;
    async fn active(executor: &mut E, id: u64) -> Result<u64, Error>;
    async fn active_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error>;
    async fn delete(executor: &mut E, id: u64) -> Result<u64, Error>;
    async fn delete_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error>;
}
