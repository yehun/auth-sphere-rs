use crate::core::DatabaseExecutor;
use crate::table::user::UserId;
use crate::table::user_password::{UserPassword, UserPasswordInsert, UserPasswordRepository, UserPasswordUpdate};
use crate::Repository;
use sqlx::{Error, Execute, QueryBuilder, Sqlite};
use tracing::debug;
use lib_sqlx::Param;

const TABLE_NAME: &'static str = "user_password";

impl<C: DatabaseExecutor> Repository<C, UserPassword, u64> for UserPassword {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserPasswordRepository<E> for UserPassword {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserPassword>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and user_id=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_password get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn get_by_password(executor: &mut E, password: &str) -> Result<Option<UserPassword>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and password=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![password.into()];
        debug!("user_password get_by_password sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, user: UserPasswordInsert) -> Result<u64, Error> {
        let sql = format!(
            "insert into {}(user_id,password) values(?,?)",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user.user_id.into(),
            user.password.into(),
        ];
        debug!("user_password insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn update(executor: &mut E, update: UserPasswordUpdate) -> Result<u64, Error> {
        let sql = format!(
            "update {} set password=? where user_id=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            update.password.into(),
            update.user_id.into(),
        ];
        debug!("user_password update sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn list_by_user_id(executor: &mut E, user_ids: &[UserId]) -> Result<Vec<UserPassword>, Error> {
        let sql = format!("select * from {} where user_id in(?", TABLE_NAME);
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(sql);
        let mut separated = builder.separated(",");
        for value_type in user_ids.iter() {
            separated.push_bind(*value_type as i64);
        }
        separated.push_unseparated(") and deleted=0");
        let builder = builder.build();
        let sql = builder.sql();
        debug!("user_password list_by_user_id sql={sql}");
        executor.list(&sql, None).await
    }
}