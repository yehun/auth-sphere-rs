use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{ColumnIndex, Database, Executor, FromRow, IntoArguments};
use crate::executor::query::ExecutorBuilderWithQuery;
use crate::ext::sql::{builder, Param};

#[allow(async_fn_in_trait)]
pub trait ExecutorBuilderWithSql<DB: Database> {
    async fn insert(&self, sql: &str, args: Option<&[Param]>) -> anyhow::Result<DB::QueryResult>;

    async fn update(&self, sql: &str, args: Option<&[Param]>) -> anyhow::Result<DB::QueryResult>;

    async fn delete(&self, sql: &str, args: Option<&[Param]>) -> anyhow::Result<DB::QueryResult>;

    async fn list<'q, O>(&self, sql: &'q str, args: Option<&[Param]>) -> anyhow::Result<Vec<O>>
    where
        O: Send + Unpin + 'q,
        O: sqlx::Type<DB>,
        O: sqlx::Encode<'q, DB>,
        O: for<'o> FromRow<'o, DB::Row>;

    async fn list_row(&self, sql: &str, args: Option<&[Param]>) -> anyhow::Result<Vec<DB::Row>>;
    async fn first<'q, O>(&self, sql: &'q str, args: Option<&[Param]>) -> anyhow::Result<Option<O>>
    where
        O: Send + Unpin + 'q,
        O: sqlx::Type<DB>,
        O: sqlx::Encode<'q, DB>,
        O: for<'o> FromRow<'o, DB::Row>;

    async fn first_row(&self, sql: &str, args: Option<&[Param]>) -> anyhow::Result<Option<DB::Row>>;

    async fn scalar<'q, O>(&self, sql: &'q str, args: Option<&[Param]>) -> anyhow::Result<Option<O>>
    where
        O: Send + Unpin + 'q,
        O: sqlx::Type<DB>,
        O: for<'d> sqlx::Decode<'d, DB>,
        (O,): for<'r> FromRow<'r, DB::Row>,
        usize: ColumnIndex<DB::Row>;
}
