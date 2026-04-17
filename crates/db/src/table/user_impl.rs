use sqlx::Error;
use tracing::debug;
use lib_sqlx::{PaginatedParam, Param};
use crate::base::Repository;
use crate::core::DatabaseExecutor;
use crate::table::user::{User, UserId, UserInsert, UserKind, UserPageResult, UserPageSearch, UserRepository, UserSearch, UserStatus, UserUpdateInfo};

impl From<UserKind> for Param {
    fn from(value: UserKind) -> Self {
        Param::Int((value as u8) as i32)
    }
}

impl From<UserKind> for u8 {
    fn from(value: UserKind) -> Self {
        value as u8
    }
}

impl From<u8> for UserKind {
    fn from(value: u8) -> Self {
        match value {
            1 => UserKind::Member,
            2 => UserKind::Community,
            3 => UserKind::Platform,
            _ => UserKind::default()
        }
    }
}

impl From<UserStatus> for Param {
    fn from(value: UserStatus) -> Self {
        Param::Int((value as u8) as i32)
    }
}

impl From<UserStatus> for u8 {
    fn from(value: UserStatus) -> Self {
        value as u8
    }
}

impl From<u8> for UserStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => UserStatus::Normal,
            2 => UserStatus::Inactive,
            3 => UserStatus::Banned,
            _ => UserStatus::Unknown
        }
    }
}

const TABLE_NAME: &'static str = "user";

impl User {
    pub fn status(&self) -> UserStatus {
        self.status.into()
    }
}


impl<C: DatabaseExecutor> Repository<C, User, UserId> for User {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserRepository<E> for User {
    async fn insert(executor: &mut E, user: UserInsert) -> Result<(u64, UserId), Error> {
        let sql = format!(
            "insert into {}(kind,nickname,username,status) values(?,?,?,?)",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user.kind.into(),
            user.nickname.into(),
            user.username.into(),
            user.status.into()
            // UserStatus::Unknown.into()
        ];
        debug!("sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok((r.rows_affected(), r.last_insert_rowid() as UserId))
        })?
    }

    async fn update_info(executor: &mut E, user: UserUpdateInfo) -> Result<u64, Error> {
        // if user.id.is_none() {
        //     return Err(Error::Decode(Box::new("id is none".to_string())));
        // }
        let sql = format!("update {} set nickname=?,username=?,status=? where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![
            user.nickname.into(),
            user.username.into(),
            user.status.into(),
            user.id.into()
        ];
        debug!("sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn update_mfa(executor: &mut E, id: UserId, active: bool) -> Result<u64, Error> {
        let sql = format!("update {} set is_mfa=? where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![
            active.into(),
            id.into()
        ];
        debug!("sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn update_passkey(executor: &mut E, id: UserId, active: bool) -> Result<u64, Error> {
        let sql = format!("update {} set is_passkey=? where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![
            active.into(),
            id.into()
        ];
        debug!("sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn update_status(executor: &mut E, id: u64, status: UserStatus) -> Result<u64, Error> {
        let sql = format!("update {} set status=? where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![
            // (status as u8).into(),
            status.into(),
            id.into()
        ];
        debug!("sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn get_by_id(executor: &mut E, id: UserId) -> Result<Option<User>, Error> {
        let sql = format!("select * from {} where deleted=0 and id=?", TABLE_NAME);
        let params: Vec<Param> = vec![id.into()];
        debug!("sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn get_by_username(executor: &mut E, username: &str) -> Result<Option<User>, Error> {
        let sql = format!("select * from {} where deleted=0 and username=?", TABLE_NAME);
        let params: Vec<Param> = vec![username.into()];
        debug!("sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn list_search(executor: &mut E, search: UserSearch) -> Result<Vec<User>, Error> {
        let mut sql = format!("select * from {} where deleted=0", TABLE_NAME);
        let mut params: Vec<Param> = vec![];
        if let Some(nickname) = search.nickname {
            sql.push_str(" and nickname like '%?%'");
            params.push(nickname.into());
        }
        if let Some(username) = search.username {
            sql.push_str(" and username like '%?%'");
            params.push(username.into());
        }
        if let Some(status) = search.status {
            sql.push_str(" and status=?");
            params.push(status.into());
        }
        debug!("sql={sql}, param={params:?}");
        executor.list(&sql, Some(&params)).await
    }

    async fn list_page(executor: &mut E, search: UserPageSearch) -> Result<UserPageResult, Error> {
        let mut sql = format!("select * from {} where deleted=0", TABLE_NAME);
        let mut params: Vec<Param> = vec![];
        if let Some(search) = search.search {
            if let Some(nickname) = search.nickname {
                sql.push_str(" and nickname like ?");
                params.push(format!("%{}%", &nickname).into());
            }
            if let Some(username) = search.username {
                sql.push_str(" and username like ?");
                params.push(format!("%{}%", &username).into());
            }
            if let Some(status) = search.status {
                sql.push_str(" and status=?");
                params.push(status.into());
            }
        }
        sql.push_str(" order by id desc");
        let param = PaginatedParam {
            params: Some(&params),
            page: Some(search.page.unwrap_or(1)),
            size: Some(search.size.unwrap_or(10)),
        };
        debug!("sql={sql}, param={param:?}");
        executor.list_page(&sql, param).await
    }

}