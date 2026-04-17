
use sqlx::{ColumnIndex, Database, Decode, Encode, Executor, FromRow, IntoArguments, Pool, Type};
use sqlx::query::{Query, QueryAs, QueryScalar};
use crate::executor::query::ExecutorBuilderWithQuery;

impl<DB> ExecutorBuilderWithQuery for Pool<DB>
where
    DB: Database,
    for<'c> <DB as Database>::Arguments<'c>: IntoArguments<'c, DB>,
    for<'c> &'c mut <DB as Database>::Connection: Executor<'c, Database=DB>
{
    type Database = DB;

    async fn execute_with_query<'q>(&self, query: Query<'q, DB, DB::Arguments<'q>>) -> anyhow::Result<DB::QueryResult> {
        query.execute(self).await.map_err(Into::into)
    }

    async fn list_with_query<'q, O>(&self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>) -> anyhow::Result<Vec<O>>
    where
        O: Send + Unpin + 'q,
        O: Type<DB>,
        O: Encode<'q, DB>,
        O: for<'o> FromRow<'o, DB::Row>
    {
        query.fetch_all(self).await.map_err(Into::into)
    }

    async fn list_row_with_query<'q>(&self, query: Query<'q, DB, DB::Arguments<'q>>) -> anyhow::Result<Vec<DB::Row>> {
        query.fetch_all(self).await.map_err(Into::into)
    }

    async fn first_with_query<'q, O>(&self, query: QueryAs<'q, DB, O, DB::Arguments<'q>>) -> anyhow::Result<Option<O>>
    where
        O: Send + Unpin + 'q,
        O: Type<DB>,
        O: Encode<'q, DB>,
        O: for<'o> FromRow<'o, DB::Row>
    {
        query.fetch_optional(self).await.map_err(Into::into)
    }

    async fn first_row_with_query<'q>(&self, query: Query<'q, DB, DB::Arguments<'q>>) -> anyhow::Result<Option<DB::Row>> {
        query.fetch_optional(self).await.map_err(Into::into)
    }

    async fn scalar_with_query<'q, O>(&self, query: QueryScalar<'q, DB, O, DB::Arguments<'q>>) -> anyhow::Result<Option<O>>
    where
        O: Send + Unpin + 'q,
        O: Type<DB>,
        O: Encode<'q, DB>,
        O: Decode<'q, DB>,
        (O,): for<'r> FromRow<'r, DB::Row>,
        usize: ColumnIndex<DB::Row>
    {
        query.fetch_optional(self).await.map_err(Into::into)
    }
}
