use crate::core::DatabaseExecutor;
use crate::table::user::UserId;
use crate::Repository;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Error;

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserPassword {
    pub id: u64,
    pub user_id: UserId,
    pub password: String,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserPasswordInsert {
    pub user_id: UserId,
    pub password: String,
}

pub struct UserPasswordUpdate {
    pub user_id: UserId,
    pub password: String,
}


#[allow(async_fn_in_trait)]
pub trait UserPasswordRepository<E: DatabaseExecutor>: Repository<E, UserPassword, u64> {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserPassword>, Error>;
    async fn get_by_password(executor: &mut E, password: &str) -> Result<Option<UserPassword>, Error>;

    async fn insert(executor: &mut E, user: UserPasswordInsert) -> Result<u64, Error>;

    async fn update(executor: &mut E, update: UserPasswordUpdate) -> Result<u64, Error>;

    async fn list_by_user_id(conn: &mut E, user_ids: &[UserId]) -> Result<Vec<UserPassword>, Error>;
}
