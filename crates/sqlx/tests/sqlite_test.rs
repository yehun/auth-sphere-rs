use anyhow::Result;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Acquire, Column, Execute, FromRow, QueryBuilder, Row, Sqlite};
use lib_sqlx::{ExecutorWith, PoolTransaction, SqlxService};
use lib_sqlx::ext::PaginatedParam;

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

#[tokio::test]
async fn test_sqlx() -> Result<()> {
    let uri = "sqlite:///home/yehun/.yongyou/data/20230926114853.db";
    // let pool = SqlitePool::connect(uri).await?;
    // let service = SqlxService::from_pool(pool).await?;
    let service: SqlxService<Sqlite> = SqlxService::from_uri(uri)?;
    let sql = r#"select * from sheet5 where id=?"#;
    // let data = service.first::<SqliteRow>(sql).await?;
    let data = service.pool().list_row(sql, Some(&vec![
        1.into()
    ])).await?;
    data.iter().for_each(|x| {
        println!("{:?}", x.get::<i32, &str>("id"));
        x.columns().iter().for_each(|c| {
            println!("{:?}", c.name());
        });
    });

    let data = service.pool().first_row(sql, Some(&vec![
        1.into()
    ])).await?;
    if let Some(row) = data {
        println!("{:?}", row.get::<i32, &str>("id"));
    }
    let sql = r#"select A from sheet5 where id=?"#;
    let data = service.pool().scalar::<String>(sql, Some(&vec![
        1.into()
    ])).await?;
    println!("{:?}", data);
    Ok(())
}

#[tokio::test]
async fn test_query() -> Result<()> {
    let uri = "sqlite:///home/yehun/.yongyou/data/20230926114853.db";
    let service: SqlxService<Sqlite> = SqlxService::from_uri(uri)?;
    // let mut conn = service.connection().await?;
    let sql = r#"select * from sheet5 where id<? order by id desc"#;
    // let data = service.list::<'_, Test>(sql, Some(&vec![
    //     Param::Int(10)
    // ])).await?;
    // let query = sqlx::query_as::<_, Test>(sql).bind(10);
    // let query = SqlxService::build_query_as::<i32, Test>(sql, vec![10]);
    // let data = query.fetch_all(&mut *conn).await?;

    let binding = vec![
        100.into()
    ];
    let param = PaginatedParam {
        page: Some(2),
        size: Some(10),
        params: Some(&binding)
    };

    let paginated = service.pool().list_page::<Test>(sql, param).await?;
    let data = paginated.data;
    println!("page: {}, size: {}", paginated.page, paginated.size);
    data.iter().for_each(|x| {
        println!("{:?}", x);
    });
    Ok(())
}

#[tokio::test]
async fn test_query_in() -> Result<()> {
    let uri = "sqlite:///home/yehun/.yongyou/data/20230926114853.db";
    let service: SqlxService<Sqlite> = SqlxService::from_uri(uri)?;
    let mut conn = service.connection().await?;

    let sql = r#"select * from sheet5 where id in("#;
    // conn.list_with_row(sql, None);
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(sql);
    let mut separated = builder.separated(",");
    let ids = vec![1, 6, 7];
    for value_type in ids.iter() {
        separated.push_bind(value_type);
    }
    separated.push_unseparated(")");
    let mut builder = builder.build();
    let sql = builder.sql();
    let args = builder.take_arguments().unwrap().unwrap();
    println!("{}", sql);
    println!("{:?}", args);

    let query = sqlx::query_as_with::<_, Test, SqliteArguments>(sql, args);
    // let data = query.fetch_all(&mut *conn).await?;
    let data = conn.list_with_query::<Test>(query).await?;
    // let data = conn.list_with_args::<Test>(sql, args).await?;
    data.iter().for_each(|x| {
        println!("{:?}", x);
    });
    Ok(())
}

#[tokio::test]
async fn test_sqlx_transaction() -> Result<()> {
    let uri = "sqlite:///home/yehun/.yongyou/data/20230926114853.db";
    let service: SqlxService<Sqlite> = SqlxService::from_uri(uri)?;
    let pool = service.pool();
    pool.transaction(|tx| Box::pin(async move {
        let sql = "update sheet5 set F=? where id=?";
        let params = vec![
            ("test1", 1),
            ("test3", 3),
            ("test5", 5),
            ("test6", 6),
        ];
        let mut row = 0;
        let conn = tx.acquire().await?;
        for (value, id) in params {
            row += sqlx::query(sql)
                .bind(value)
                .bind(id)
                .execute(&mut *conn)
                .await?
                .rows_affected();
        }
        println!("{}", row);
        Ok(row as usize)
    })).await?;
    // let tx = connection.begin().await?;
    // tx.acquire().await?;
    Ok(())
}