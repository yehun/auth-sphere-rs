use sqlx::pool::PoolConnection;
use sqlx::{Connection, Database, Error, Pool};

pub struct SqlxService<DB: Database> {
	pool: Pool<DB>,
}

impl<DB: Database> SqlxService<DB> {
	pub fn from_uri(uri: &str) -> Result<Self, Error> {
		let pool = Pool::connect_lazy(uri)?;
		Ok(Self { pool })
	}
	pub async fn async_from_uri(uri: &str) -> Result<Self, Error> {
		let pool = Pool::connect(uri).await?;
		Ok(Self { pool })
	}

	pub fn from_pool(pool: Pool<DB>) -> Result<Self, Error> {
		Ok(Self {
			pool: pool.clone()
		})
	}

	pub fn pool(&self) -> Pool<DB> {
		self.pool.clone()
	}

	pub async fn connection(&self) -> Result<PoolConnection<DB>, Error> {
		Ok(self.pool.acquire().await?)
	}

	pub async fn ping(&self) -> Result<(), Error> {
		let mut connection = self.connection().await?;
		connection.ping().await
	}

	// pub async fn transaction<'a, F>(&'a self, callback: F) -> Result<usize>
	// where
	// 	F: Send + Sync + 'a,
	// 	for<'c> F: FnOnce(&'c mut Transaction<'_, DB>) -> BoxFuture<'c, Result<usize>>
	// {
	// 	let mut conn = self.pool.acquire().await?;
	// 	Ok(conn.transaction(callback).await?)
	// }
	//
	// pub async fn transaction_execute<F, R>(&self, callback: F) -> Result<R>
	// where
	// 	for<'c> F: FnOnce(&'c mut Transaction<'_, DB>) -> Pin<Box<dyn Future<Output = Result<R>> + Send + 'c>> + Send,
	// 	R: Send,
	// {
	// 	let mut conn = self.pool.acquire().await?;
	// 	let mut transaction = conn.begin().await?;
	// 	let result = callback(&mut transaction).await;
	// 	match result {
	// 		Ok(r) => {
	// 			transaction.commit().await?;
	// 			Ok(r)
	// 		}
	// 		Err(e) => {
	// 			transaction.rollback().await?;
	// 			Err(e)
	// 		}
	// 	}
	// }
}