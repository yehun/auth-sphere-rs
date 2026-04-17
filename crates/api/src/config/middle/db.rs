use auth_sphere_db::{
    core::DatabasePool, 
    create_database_pool
};

use crate::config::application;

pub(crate) async fn init() -> DatabasePool {
    let config = application::get();
    if let Err(e) = auth_sphere_db::init_database(config.database.path.as_str()).await {
        panic!("init db error: {e:?}")
    }
    let uri = format!("sqlite://{}", config.database.path);
    let max_connections = config.database.max_connections;
    create_database_pool(&uri, max_connections).await.expect("init db pool error")
}



// pub type Component<T> = Pin<Box<dyn Future<Output = T> + Send>>;
// #[bean]
// fn build_db_pool() -> Component<DatabasePool> {
//     Box::pin(async move {
//         init().await
//     })
//     // let result = thread::spawn(move || {
//     //     Runtime::new().unwrap().block_on(async {
//     //         init().await
//     //     })
//     // }).join().unwrap_or_else(|e| panic!("initial db pool runtime error: {e:?}"));
//     // match result {
//     //     Ok(pool) => Some(pool),
//     //     Err(e) => panic!("initial db pool error: {e:?}")
//     // }
// }