use lib_sqlx::Param;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow};
use crate::core::{DatabaseExecutor, DatabaseQueryResult};

#[derive(Debug, Clone, Default)]
pub struct PaginateSearch<T> {
    pub search: Option<T>,
    pub page: Option<u32>,
    pub size: Option<u64>
}

#[allow(async_fn_in_trait, private_bounds)]
pub trait Repository<E, T, PK>
where
    E: DatabaseExecutor,
    T: Send + Unpin + for<'o> FromRow<'o, SqliteRow>,
    Param: From<PK>
{
    fn table_name() -> &'static str;

    async fn execute(executor: &mut E, sql: &str, params: Option<&[Param]>) -> Result<DatabaseQueryResult, Error> {
        executor.execute_with_sql(sql, params).await
    }

    async fn delete(executor: &mut E, id: PK) -> Result<u64, Error> {
        let sql = format!("update {} set deleted=1 where deleted=0 and id=?", Self::table_name());
        let params = vec![Param::from(id)];
        executor.execute_with_sql(&sql, Some(&params)).await.map(|x| {
            Ok(x.rows_affected())
        })?
    }

    async fn list_all(executor: &mut E) -> Result<Option<T>, Error> {
        let sql = format!("select * from {} where deleted=0", Self::table_name());
        executor.first::<T>(&sql, None).await
    }

    async fn get(executor: &mut E, id: PK) -> Result<Option<T>, Error> {
        let sql = format!("select * from {} where deleted=0 and id=?", Self::table_name());
        let param = vec![Param::from(id)];
        executor.first::<T>(&sql, Some(&param)).await
    }

}
