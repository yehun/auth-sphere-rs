use tracing::debug;
use lib_sqlx::Param;
use lib_sqlx::sqlx::Error;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;
use crate::table::user_session::{UserDevice, UserSession, UserSessionInsert, UserSessionRepository};

impl From<UserDevice> for Param {
    fn from(value: UserDevice) -> Self {
        Param::Int((value as u8) as i32)
    }
}

impl From<UserDevice> for u8 {
    fn from(value: UserDevice) -> Self {
        value as u8
    }
}

impl From<u8> for UserDevice {
    fn from(value: u8) -> Self {
        match value {
            1 => UserDevice::Web,
            2 => UserDevice::Android,
            3 => UserDevice::Ios,
            4 => UserDevice::Desktop,
            _ => UserDevice::Web
        }
    }
}


const TABLE_NAME: &'static str = "user_session";

impl<C: DatabaseExecutor> Repository<C, UserSession, u64> for UserSession {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserSessionRepository<E> for UserSession {
    async fn get_by_token(executor: &mut E, token: &str) -> Result<Option<UserSession>, Error> {
        let sql = format!("select * from {} where deleted=0 and token=?", TABLE_NAME);
        let params: Vec<Param> = vec![token.into()];
        debug!("user_session get_by_token sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }
    async fn get_by_user_id(executor: &mut E, user_id: UserId, device: UserDevice) -> Result<Option<UserSession>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and user_id=? and device=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user_id.into(),
            device.into(),
        ];
        debug!("user_session get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, user: UserSessionInsert) -> Result<u64, Error> {
        let sql = format!(
            "insert into {}(user_id,device,token) values(?,?,?)",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user.user_id.into(),
            user.device.into(),
            user.token.into()
        ];
        debug!("user_session insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn delete(executor: &mut E, id: u64) -> Result<u64, Error> {
        let sql = format!("update {} set deleted=1 where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![id.into()];
        debug!("user_session delete sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }
}