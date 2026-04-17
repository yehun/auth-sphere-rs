use anyhow::Result;
use lib_sqlx::{PoolTransaction, SqlxService};
use sqlx::{Acquire, FromRow, Sqlite, SqlitePool};
use lib_sqlx::ext::{ExecutorWith, Param};

#[derive(Debug, Clone, FromRow)]
struct Test {
    id: i32,
    #[sqlx(rename = "A")]
    name_py: String,
    #[sqlx(rename = "B")]
    name: String,
    #[sqlx(rename = "C")]
    email: String,
    #[sqlx(rename = "D")]
    nick_name: String,
    #[sqlx(rename = "E")]
    status: String,
    #[sqlx(rename = "F")]
    department: Option<String>,
}


fn get_db_pool() -> Result<SqlitePool> {
    let uri = "sqlite:///home/yehun/.yongyou/data/20230926114853.db";
    Ok(SqlxService::from_uri(uri)?.pool())
}

#[tokio::test]
async fn test_execute() -> Result<()> {
    let mut pool = get_db_pool()?;
    let sql = r#"select * from sheet5 where id=?"#;
    let query = sqlx::query_as::<Sqlite, Test>(sql).bind(1);
    let data = pool.list_with_query::<Test>(query).await?;
    data.iter().for_each(|x| {
        println!("{:?}", x)
    });
    let data = pool.list::<Test>(sql, Some(&vec![
        1.into()
    ])).await?;
    data.iter().for_each(|x| {
        println!("{:?}", x)
    });
    // println!("{:?}", data);
    Ok(())
}


#[tokio::test]
async fn test_execute_transaction() -> Result<()> {
    let pool = get_db_pool()?;
    let row = pool.transaction(|tx| Box::pin(async move {
        let conn = tx.acquire().await?;
        let mut row = 0;
        let sql = "update sheet5 set F=? where id=?";
        let params: Vec<Vec<Param>> = vec![
            vec!["test1".into(), 1.into()],
            vec!["test3".into(), 3.into()],
            vec!["test5".into(), 5.into()],
            vec!["test6".into(), 6.into()],
        ];

        for (i, param) in params.iter().enumerate() {
            if i == 3 {
                return Err(anyhow::anyhow!("1234567890-"));
            }
            let result = conn.execute_with_sql(sql, Some(param)).await;
            match result {
                Ok(r) => row += r.rows_affected(),
                Err(e) => {
                    println!("error: {}", e);
                    return Err(e.into());
                }
            }
        }
        Ok(row as usize)
    })).await?;

    println!("total_rows_affected: {}", row);
    // let tx = connection.begin().await?;
    // tx.acquire().await?;
    Ok(())
}