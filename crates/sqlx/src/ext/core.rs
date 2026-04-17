use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{ColumnIndex, Database, FromRow, IntoArguments, Type, Encode, Decode, Error};
use sqlx::query::{Query, QueryAs, QueryScalar};
use crate::{Paginated, PaginatedParam};
use crate::Param;

#[allow(async_fn_in_trait)]
pub trait ExecutorWith<DB>
where
    DB: Database,
    for<'c> DB::Arguments<'c>: IntoArguments<'c, DB>,
    for<'p> String: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> bool: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> i16: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> i32: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> i64: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> f32: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> f64: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> NaiveDate: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> NaiveDateTime: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
    for<'p> Vec<u8>: Encode<'p, DB> + Decode<'p, DB> + Type<DB>,
{
    async fn execute_with_sql(&mut self, sql: &str, args: Option<&[Param]>) -> Result<DB::QueryResult, Error>;

    async fn list<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Vec<O>, Error>
    where
        O: Send + Unpin + 'q,
        O: for<'o> FromRow<'o, DB::Row>;

    async fn list_page<'q, O>(&mut self, sql: &'q str, param: PaginatedParam<'q>) -> Result<Paginated<O>, Error>
    where
        O: Send + Unpin + 'q,
        O: for<'o> FromRow<'o, DB::Row>,
        u64: Type<DB>,
        for<'d> u64: Decode<'d, DB>,
        usize: ColumnIndex<<DB as Database>::Row>;

    async fn list_row(&mut self, sql: &str, args: Option<&[Param]>) -> Result<Vec<DB::Row>, Error>;

    async fn first<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<O>, Error>
    where
        O: Send + Unpin + 'q,
        O: for<'o> FromRow<'o, DB::Row>;

    async fn first_row(&mut self, sql: &str, args: Option<&[Param]>) -> Result<Option<DB::Row>, Error>;

    async fn scalar<'q, O>(&mut self, sql: &'q str, args: Option<&[Param]>) -> Result<Option<O>, Error>
    where
        O: Send + Unpin + 'q,
        O: Type<DB>,
        O: for<'d> Decode<'d, DB>,
        (O,): for<'o> FromRow<'o, DB::Row>,
        usize: ColumnIndex<DB::Row>;

    async fn execute_with_query<'q>(&mut self, query: Query<'q, DB, DB::Arguments<'q>>) -> Result<DB::QueryResult, Error>;

    async fn list_with_query<'q, O>(&mut self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>) -> Result<Vec<O>, Error>
    where
        O: Send + Unpin + 'q,
        O: for<'o> FromRow<'o, DB::Row>;

    async fn list_row_with_query<'q>(&mut self, query: Query<'q, DB, DB::Arguments<'q>>) -> Result<Vec<DB::Row>, Error>;

    async fn first_with_query<'q, O>(&mut self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>) -> Result<Option<O>, Error>
    where
        O: Send + Unpin + 'q,
        O: for<'o> FromRow<'o, DB::Row>;

    async fn first_row_with_query<'q>(&mut self, query: Query<'q, DB, DB::Arguments<'q>>) -> Result<Option<DB::Row>, Error>;

    async fn scalar_with_query<'q, O>(&mut self, query: QueryScalar<'q, DB, O, DB::Arguments<'q>>) -> Result<Option<O>, Error>
    where
        O: Send + Unpin + 'q,
        (O,): for<'o> FromRow<'o, DB::Row>,
        usize: ColumnIndex<DB::Row>,;
}
