use crate::base::{PaginateSearch, Repository};
use crate::core::DatabaseExecutor;
use chrono::NaiveDateTime;
use lib_sqlx::Paginated;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sqlx::Error;
use strum_macros::EnumIter;

pub type UserId = u64;


#[repr(u8)]
#[derive(Debug, Clone, Default, PartialEq, EnumIter, Serialize_repr, Deserialize_repr)]
pub enum UserKind {
    #[default]
    Member = 1,
    Community = 2,
    Platform = 3
}

#[repr(u8)]
#[derive(Debug, Clone, Default, PartialEq, EnumIter, Serialize_repr, Deserialize_repr)]
pub enum UserStatus {
    #[default]
    Normal = 1,
    Inactive = 2,
    Banned = 3,
    Unknown = 0,
}

#[derive(Debug, Clone, Default, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub kind: u8,
    pub nickname: String,
    pub username: String,
    pub is_mfa: bool,
    pub status: u8,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

pub struct UserInsert {
    pub kind: UserKind,
    pub nickname: String,
    pub username: String,
    pub status: UserStatus,
}

pub struct UserUpdateInfo {
    pub id: UserId,
    pub nickname: String,
    pub username: String,
    pub status: UserStatus,
}

pub struct UserUpdatePassword {
    pub id: UserId,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct UserSearch {
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub status: Option<UserStatus>,
}

pub type UserPageSearch = PaginateSearch<UserSearch>;
pub type UserPageResult = Paginated<User>;

#[allow(async_fn_in_trait)]
pub trait UserRepository<E: DatabaseExecutor>: Repository<E, User, UserId> {
    async fn insert(executor: &mut E, user: UserInsert) -> Result<(u64, UserId), Error>;

    async fn update_info(executor: &mut E, user: UserUpdateInfo) -> Result<u64, Error>;

    async fn update_mfa(executor: &mut E, id: UserId, active: bool) -> Result<u64, Error>;

    async fn update_status(executor: &mut E, id: u64, status: UserStatus) -> Result<u64, Error>;

    async fn get_by_id(executor: &mut E, id: UserId) -> Result<Option<User>, Error>;

    async fn get_by_username(executor: &mut E, username: String) -> Result<Option<User>, Error>;

    async fn list_search(executor: &mut E, search: UserSearch) -> Result<Vec<User>, Error>;

    async fn list_page(executor: &mut E, search: UserPageSearch) -> Result<Paginated<User>, Error>;
}
