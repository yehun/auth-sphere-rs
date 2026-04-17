use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserEmail {
    pub id: u64,
    pub user_id: UserId,
    pub email: String,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserEmailInsert {
    pub user_id: UserId,
    pub email: String,
}


pub struct UserEmailUpdate {
    pub user_id: UserId,
    pub email: String,
}

#[allow(async_fn_in_trait)]
pub trait UserEmailRepository<E: DatabaseExecutor>: Repository<E, UserEmail, u64> {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserEmail>, Error>;
    async fn get_by_email(executor: &mut E, email: &str) -> Result<Option<UserEmail>, Error>;
    async fn insert(executor: &mut E, user: UserEmailInsert) -> Result<u64, Error>;

    async fn update(executor: &mut E, update: UserEmailUpdate) -> Result<u64, Error>;
    async fn list_by_user_id(conn: &mut E, user_ids: &[UserId]) -> Result<Vec<UserEmail>, Error>;
}
