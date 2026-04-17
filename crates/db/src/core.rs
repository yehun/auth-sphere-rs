use std::str::FromStr;
use sqlx::{Error, Pool, Sqlite, SqliteConnection};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteQueryResult};
use lib_sqlx::{ExecutorWith, SqlxService};

pub type DatabaseService = SqlxService<Sqlite>;

pub type DatabasePool = Pool<Sqlite>;

pub type DatabasePoolConnection = PoolConnection<Sqlite>;

pub type DatabaseConnection = SqliteConnection;

pub type DatabaseQueryResult = SqliteQueryResult;


pub trait DatabaseExecutor: ExecutorWith<Sqlite> {}

impl DatabaseExecutor for DatabasePool {}
impl DatabaseExecutor for DatabasePoolConnection {}
impl DatabaseExecutor for DatabaseConnection {}

pub async fn create_database_pool(uri: &str, max_connections: u32) -> Result<DatabasePool, Error> {
    let options = SqliteConnectOptions::from_str(&uri)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options)
        .await?;
    let service: DatabaseService = SqlxService::from_pool(pool)?;
    Ok(service.pool())
}