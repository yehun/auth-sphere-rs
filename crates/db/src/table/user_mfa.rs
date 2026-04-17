use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserMfa {
    pub id: u64,
    pub user_id: UserId,
    pub secret: String,
    pub active: bool,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserMfaUpdate {
    pub user_id: UserId,
    pub secret: String,
}

#[allow(async_fn_in_trait)]
pub trait UserMfaRepository<E: DatabaseExecutor>: Repository<E, UserMfa, u64> {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserMfa>, Error>;
    async fn insert(executor: &mut E, user: UserMfaUpdate) -> Result<u64, Error>;
    async fn active(executor: &mut E, id: u64) -> Result<u64, Error>;
    async fn active_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error>;
    async fn delete(executor: &mut E, id: u64) -> Result<u64, Error>;
    async fn delete_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error>;
}
