use sqlx::{Error, Execute, QueryBuilder, Sqlite};
use tracing::debug;
use lib_sqlx::Param;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;
use crate::table::user_email::{UserEmail, UserEmailInsert, UserEmailRepository, UserEmailUpdate};

const TABLE_NAME: &'static str = "user_email";

impl<C: DatabaseExecutor> Repository<C, UserEmail, u64> for UserEmail {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserEmailRepository<E> for UserEmail {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserEmail>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and user_id=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_email get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn get_by_email(executor: &mut E, email: &str) -> Result<Option<UserEmail>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and email=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![email.into()];
        debug!("user_email get_by_email sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, user: UserEmailInsert) -> Result<u64, Error> {
        let sql = format!(
            "insert into {}(user_id,email) values(?,?)",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user.user_id.into(),
            user.email.into(),
        ];
        debug!("user_email insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn update(executor: &mut E, update: UserEmailUpdate) -> Result<u64, Error> {
        let sql = format!(
            "update {} set email=? where user_id=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            update.email.into(),
            update.user_id.into(),
        ];
        debug!("user_email update sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn list_by_user_id(executor: &mut E, user_ids: &[UserId]) -> Result<Vec<UserEmail>, Error> {
        let sql = format!("select * from {} where user_id in(?", TABLE_NAME);
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(sql);
        let mut separated = builder.separated(",");
        for value_type in user_ids.iter() {
            separated.push_bind(*value_type as i64);
        }
        separated.push_unseparated(") and deleted=0");
        let builder = builder.build();
        let sql = builder.sql();
        debug!("user_email list_by_user_id sql={sql}");
        executor.list(&sql, None).await
    }
}