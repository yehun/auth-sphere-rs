use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserPhone {
    pub id: u64,
    pub user_id: UserId,
    pub phone: String,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserPhoneInsert {
    pub user_id: UserId,
    pub phone: String,
}

pub struct UserPhoneUpdate {
    pub user_id: UserId,
    pub phone: String,
}

#[allow(async_fn_in_trait)]
pub trait UserPhoneRepository<E: DatabaseExecutor>: Repository<E, UserPhone, u64> {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserPhone>, Error>;
    async fn get_by_phone(executor: &mut E, phone: &str) -> Result<Option<UserPhone>, Error>;
    async fn insert(executor: &mut E, user: UserPhoneInsert) -> Result<u64, Error>;
    async fn update(executor: &mut E, update: UserPhoneUpdate) -> Result<u64, Error>;
    async fn list_by_user_id(conn: &mut E, user_ids: &[UserId]) -> Result<Vec<UserPhone>, Error>;
}
