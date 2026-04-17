use sqlx::{Error, Execute, QueryBuilder, Sqlite};
use tracing::debug;
use lib_sqlx::Param;
use crate::core::DatabaseExecutor;
use crate::Repository;
use crate::table::user::UserId;
use crate::table::user_phone::{UserPhone, UserPhoneInsert, UserPhoneRepository, UserPhoneUpdate};

const TABLE_NAME: &'static str = "user_phone";

impl<C: DatabaseExecutor> Repository<C, UserPhone, u64> for UserPhone {
    fn table_name() -> &'static str {
        TABLE_NAME
    }
}


impl<E: DatabaseExecutor> UserPhoneRepository<E> for UserPhone {
    async fn get_by_user_id(executor: &mut E, user_id: UserId) -> Result<Option<UserPhone>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and user_id=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![user_id.into()];
        debug!("user_phone get_by_user_id sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn get_by_phone(executor: &mut E, phone: &str) -> Result<Option<UserPhone>, Error> {
        let sql = format!(
            "select * from {} where deleted=0 and phone=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![phone.into()];
        debug!("user_phone get_by_phone sql={sql}, param={params:?}");
        executor.first(&sql, Some(&params)).await
    }

    async fn insert(executor: &mut E, user: UserPhoneInsert) -> Result<u64, Error> {
        let sql = format!(
            "insert into {}(user_id,phone) values(?,?)",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            user.user_id.into(),
            user.phone.into(),
        ];
        debug!("user_phone insert sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn update(executor: &mut E, update: UserPhoneUpdate) -> Result<u64, Error> {
        let sql = format!(
            "update {} set phone=? where user_id=?",
            TABLE_NAME
        );
        let params: Vec<Param> = vec![
            update.phone.into(),
            update.user_id.into(),
        ];
        debug!("user_phone update sql={sql}, param={params:?}");
        Self::execute(executor, &sql, Some(&params)).await.map(|r| {
            Ok(r.rows_affected())
        })?
    }

    async fn list_by_user_id(executor: &mut E, user_ids: &[UserId]) -> Result<Vec<UserPhone>, Error> {
        let sql = format!("select * from {} where user_id in(?", TABLE_NAME);
        let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(sql);
        let mut separated = builder.separated(",");
        for value_type in user_ids.iter() {
            separated.push_bind(*value_type as i64);
        }
        separated.push_unseparated(") and deleted=0");
        let builder = builder.build();
        let sql = builder.sql();
        debug!("user_phone list_by_user_id sql={sql}");
        executor.list(&sql, None).await
    }
}