use anyhow::Result;
use futures_core::future::BoxFuture;
use sqlx::{Connection, Database, Pool, Transaction};

pub trait PoolTransaction<DB: Database> {
    fn transaction<'a, F, O>(&self, callback: F) -> impl Future<Output = Result<O>> + Send
    where
        F: Send + Sync + 'a,
        for<'c> F: FnOnce(&'c mut Transaction<'_, DB>) -> BoxFuture<'c, Result<O>>,
        O: Send;
}

impl<DB: Database> PoolTransaction<DB> for Pool<DB> {
    async fn transaction<'a, F, O>(&self, callback: F) -> Result<O>
    where
        F: Send + Sync + 'a,
        for<'c> F: FnOnce(&'c mut Transaction<'_, DB>) -> BoxFuture<'c, Result<O>>,
        O: Send
    {
        let mut conn = self.acquire().await?;
        Ok(conn.transaction(callback).await?)
    }
}
