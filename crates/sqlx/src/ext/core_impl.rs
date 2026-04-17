use sqlx::query::QueryScalar;
use sqlx::query::QueryAs;
use sqlx::query::Query;
use crate::Paginated;
use crate::PaginatedParam;
use crate::ext::builder;
use crate::Param;
use crate::ExecutorWith;
use sqlx::{
    Acquire, Database, Executor,
    IntoArguments, FromRow,
    Type, Encode, Decode, ColumnIndex,
    error::Error
};
use crate::impl_executor_with;
use sqlx::pool::Pool;
use sqlx::pool::PoolConnection;
use chrono::{NaiveDate, NaiveDateTime};

impl_executor_with!(Pool);
impl_executor_with!(PoolConnection);

use crate::impl_connection_with;

#[cfg(feature = "sqlite")]
impl_connection_with!(sqlx::Sqlite, sqlx::SqliteConnection);

#[cfg(feature = "mysql")]
impl_connection_with!(sqlx::MySql, sqlx::MySqlConnection);

// #[cfg(feature = "postgres")]
// impl_connection_with!(sqlx::Postgres, sqlx::PgConnection);