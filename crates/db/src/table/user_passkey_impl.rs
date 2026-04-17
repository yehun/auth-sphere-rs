use tracing::debug;
use lib_sqlx::Param;
use lib_sqlx::sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;
use crate::table::user_passkey::{UserPassKey, UserPassKeyInsert, UserPassKeyRepository};

const TABLE_NAME: &'static str = "user_passkey";

impl<C: DatabaseExecutor> Repository<C, UserPassKey, u64> for UserPassKey {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserPassKeyRepository<E> for UserPassKey {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserPassKey>, Error> {
        let sql = format!("select * from {} where deleted=0 and user_id=?", TABLE_NAME);
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_passkey get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn get_by_credential_id(executor: &mut E, credential_id: &str) -> Result<Option<UserPassKey>, Error> {
        let sql = format!("select * from {} where deleted=0 and credential_id=?", TABLE_NAME);
        let params: Vec<Param> = vec![credential_id.into()];
        debug!("user_passkey get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, data: UserPassKeyInsert) -> Result<u64, Error> {
        let sql = format!(r#"
            insert into {}(user_id,credential_id,public_key,sign_count,active)
            values(?,?,?,0,?)
        "#, TABLE_NAME);
        let params: Vec<Param> = vec![
            data.user_id.into(),
            data.credential_id.into(),
            data.public_key.into(),
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

    async fn delete(executor: &mut E, id: u64) -> Result<u64, Error> {
        let sql = format!("update {} set deleted=0 where deleted=1 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![id.into()];
        debug!("user_mfa delete sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn delete_by_user_id(executor: &mut E, user_id: UserId) -> Result<u64, Error> {
        let sql = format!("update {} set deleted=0 where deleted=1 and user_id=?", TABLE_NAME);
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_mfa delete_by_user_id sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }
}