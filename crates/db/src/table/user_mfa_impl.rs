use tracing::debug;
use lib_sqlx::Param;
use lib_sqlx::sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;
use crate::table::user_mfa::{UserMfa, UserMfaRepository, UserMfaUpdate};

const TABLE_NAME: &'static str = "user_mfa";

impl<C: DatabaseExecutor> Repository<C, UserMfa, u64> for UserMfa {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserMfaRepository<E> for UserMfa {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserMfa>, Error> {
        let sql = format!("select * from {} where deleted=0 and user_id=?", TABLE_NAME);
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_mfa get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, update: UserMfaUpdate) -> Result<u64, Error> {
        let sql = format!("insert into {}(user_id,secret,active) values(?,?,?)", TABLE_NAME);
        let params: Vec<Param> = vec![
            update.user_id.into(),
            update.secret.into(),
            false.into()
        ];
        debug!("user_mfa insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn active(executor: &mut E, id: u64) -> Result<u64, Error> {
        let sql = format!("update {} set active=? where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![
            true.into(),
            id.into()
        ];
        debug!("user_mfa active sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn active_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error> {
        let sql = format!("update {} set active=? where deleted=0 and user_id=?", TABLE_NAME);
        let params: Vec<Param> = vec![
            true.into(),
            user_id.into()
        ];
        debug!("user_mfa active sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn delete_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error> {
        let sql = format!("update {} set deleted=1 where deleted=0 and user_id=?", TABLE_NAME);
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_mfa delete_by_user_id sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn delete(executor: &mut E, id: u64) -> Result<u64, Error> {
        let sql = format!("update {} set deleted=1 where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![id.into()];
        debug!("user_mfa delete sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }
}