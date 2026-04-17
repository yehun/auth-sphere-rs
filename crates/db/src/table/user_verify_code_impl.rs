use sqlx::Error;
use tracing::debug;
use lib_sqlx::Param;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;
use crate::table::user_verify_code::{UserVerifyCode, UserVerifyCodeInsert, UserVerifyCodeRepository, UserVerifySourceKind};

impl From<UserVerifySourceKind> for Param {
    fn from(value: UserVerifySourceKind) -> Self {
        Param::Int((value as u8) as i32)
    }
}

impl From<UserVerifySourceKind> for u8 {
    fn from(value: UserVerifySourceKind) -> Self {
        value as u8
    }
}

impl From<u8> for UserVerifySourceKind {
    fn from(value: u8) -> Self {
        match value {
            1 => UserVerifySourceKind::Email,
            2 => UserVerifySourceKind::Phone,
            _ => UserVerifySourceKind::Email
        }
    }
}


const TABLE_NAME: &'static str = "user_verify_code";

impl<C: DatabaseExecutor> Repository<C, UserVerifyCode, u64> for UserVerifyCode {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserVerifyCodeRepository<E> for UserVerifyCode {
    async fn get(executor: &mut E, user_id: UserId, source: &str) -> Result<Option<UserVerifyCode>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and user_id=? and source=? and verify_at is null",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user_id.into(),
            source.into(),
        ];
        debug!("user_verify_code get sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, insert: UserVerifyCodeInsert) -> Result<u64, Error> {
        let sql = format!(
            "insert into {}(user_id,source_kind,source,code) values(?,?,?,?)",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            insert.user_id.into(),
            insert.source_kind.into(),
            insert.source.into(),
            insert.code.into(),
        ];
        debug!("user_phone insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn verify(executor: &mut E, user_id: UserId, source: &str) -> Result<u64, Error> {
        // datetime(CURRENT_TIMESTAMP, 'localtime')
        let sql = format!(
            "update {} set verify_at=? where deleted=0 and user_id=? and source=? and verify_at is null",
            TABLE_NAME
        );
        // chrono::NaiveDateTime::
        let now = chrono::Local::now().naive_local();
        let params: Vec<Param> = vec![
            now.into(),
            user_id.into(),
            source.into(),
        ];
        debug!("user_phone insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }
}