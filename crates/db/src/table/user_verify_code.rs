use crate::core::DatabaseExecutor;
use crate::table::user::UserId;
use crate::Repository;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::Error;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, EnumIter, Serialize_repr, Deserialize_repr)]
pub enum UserVerifySourceKind {
    Email,
    Phone,
}


#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserVerifyCode {
    pub id: u64,
    pub user_id: UserId,
    pub source_kind: u8,
    pub source: String,
    pub code: String,
    pub verify_at: Option<NaiveDateTime>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserVerifyCodeInsert {
    pub user_id: UserId,
    pub source_kind: u8,
    pub source: String,
    pub code: String,
}

pub struct UserVerifyCodeUpdate {
    pub user_id: UserId,
    pub source: String,
}

#[allow(async_fn_in_trait)]
pub trait UserVerifyCodeRepository<E: DatabaseExecutor>: Repository<E, UserVerifyCode, u64> {
    async fn get(executor: &mut E, user_id: UserId, source: &str) -> Result<Option<UserVerifyCode>, Error>;
    async fn insert(executor: &mut E, insert: UserVerifyCodeInsert) -> Result<u64, Error>;
    async fn verify(executor: &mut E, user_id: UserId, source: &str) -> Result<u64, Error>;
}
