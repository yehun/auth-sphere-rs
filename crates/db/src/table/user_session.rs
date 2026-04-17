use crate::core::DatabaseExecutor;
use crate::table::user::UserId;
use crate::Repository;
use chrono::NaiveDateTime;
use lib_sqlx::sqlx::Error;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Debug, Clone, Default, PartialEq, EnumIter, Serialize_repr, Deserialize_repr)]
pub enum UserDevice {
    #[default]
    Web = 1,
    Android = 2,
    Ios = 3,
    Desktop = 4
}

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserSession {
    pub id: u64,
    pub user_id: UserId,
    pub device: u8,
    pub token: String,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserSessionInsert {
    pub user_id: UserId,
    pub device: u8,
    pub token: String,
}



#[allow(async_fn_in_trait)]
pub trait UserSessionRepository<E: DatabaseExecutor>: Repository<E, UserSession, u64> {
    async fn get_by_token(executor: &mut E, token: &str) -> Result<Option<UserSession>, Error>;
    async fn get_by_user_id(executor: &mut E, id: UserId, device: UserDevice) -> Result<Option<UserSession>, Error>;
    async fn insert(executor: &mut E, user: UserSessionInsert) -> Result<u64, Error>;
    async fn delete(executor: &mut E, id: u64) -> Result<u64, Error>;
}

