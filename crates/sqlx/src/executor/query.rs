use anyhow::Result;
use sqlx::query::{Query, QueryAs, QueryScalar};
use sqlx::{ColumnIndex, Database, Executor, FromRow};

#[allow(async_fn_in_trait)]
pub trait ExecutorBuilderWithQuery {
    type Database: Database;

    async fn execute_with_query<'q>(&self, query: Query<'q, Self::Database, <Self::Database as Database>::Arguments<'q>>) -> Result<<Self::Database as Database>::QueryResult>;

    async fn list_with_query<'q, O>(&self, query: QueryAs<'q, Self::Database, O, <Self::Database as Database>::Arguments<'q>>) -> Result<Vec<O>>
    where
        O: Send + Unpin + 'q,
        O: sqlx::Type<Self::Database>,
        O: sqlx::Encode<'q, Self::Database>,
        O: for<'o> FromRow<'o, <Self::Database as Database>::Row>;

    async fn list_row_with_query<'q>(&self, query: Query<'q, Self::Database, <Self::Database as Database>::Arguments<'q>>) -> Result<Vec<<Self::Database as Database>::Row>>;

    async fn first_with_query<'q, O>(&self, query: QueryAs<'q, Self::Database, O, <Self::Database as Database>::Arguments<'q>>) -> Result<Option<O>>
    where
        O: Send + Unpin + 'q,
        O: sqlx::Type<Self::Database>,
        O: sqlx::Encode<'q, Self::Database>,
        O: for<'o> FromRow<'o, <Self::Database as Database>::Row>;

    async fn first_row_with_query<'q>(&self, query: Query<'q, Self::Database, <Self::Database as Database>::Arguments<'q>>) -> Result<Option<<Self::Database as Database>::Row>>;

    async fn scalar_with_query<'q, O>(&self, query: QueryScalar<'q, Self::Database, O, <Self::Database as Database>::Arguments<'q>>) -> Result<Option<O>>
    where
        O: Send + Unpin + 'q,
        O: sqlx::Type<Self::Database>,
        O: sqlx::Encode<'q, Self::Database>,
        O: sqlx::Decode<'q, Self::Database>,
        (O,): for<'r> FromRow<'r, <Self::Database as Database>::Row>,
        usize: ColumnIndex<<Self::Database as Database>::Row>;
}
