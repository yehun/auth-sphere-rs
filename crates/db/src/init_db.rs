use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

/// 初始化数据库，创建所有必要的表
pub async fn init_database(db_path: &str) -> Result<()> {
    println!("📦 正在初始化数据库: {}", db_path);
    
    // 创建数据库连接
    let uri = format!("sqlite://{}", db_path);
    let options = SqliteConnectOptions::from_str(&uri)?
        .create_if_missing(true);
    
    let pool = SqlitePool::connect_with(options).await?;

    create_table(&pool).await?;
    // create_table_data(&pool).await?;
    // 插入初始数据
    // insert_initial_data(&pool).await?;
    println!("✅ 数据库初始化完成！");
    
    pool.close().await;
    
    Ok(())
}


async fn create_table(pool: &SqlitePool) -> Result<()> {
    println!("📋 创建表...");
    const INIT_SQL: &str = include_str!("../init.sql");
    sqlx::query(INIT_SQL).execute(pool).await?;
    println!("✓ 表创建成功");
    Ok(())
}


async fn create_table_data(pool: &SqlitePool) -> Result<()> {
    println!("📋 创建表...");
    const INIT_SQL: &str = include_str!("../data.sql");
    sqlx::query(INIT_SQL).execute(pool).await?;
    println!("✓ 表创建成功");
    Ok(())
}

/// 插入初始数据
async fn insert_initial_data(pool: &SqlitePool) -> Result<()> {
    println!("📝 插入初始数据...");
    
    // 检查是否已存在该用户
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM user WHERE username = ?"
    )
    .bind("yehun")
    .fetch_one(pool)
    .await?;
    
    if existing > 0 {
        println!("⚠️  用户 'yehun' 已存在，跳过初始化数据插入");
        return Ok(());
    }
    
    // 开始事务
    let mut tx = pool.begin().await?;
    
    // 1. 插入用户（会员类型，正常状态）
    let result = sqlx::query(
        r#"
        INSERT INTO user (kind, nickname, username, status, create_at, update_at, deleted)
        VALUES (1, 'yehun', 'yehun', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
        "#
    ).execute(&mut *tx).await?;
    
    let user_id = result.last_insert_rowid();
    println!("✓ 插入用户: yehun (ID: {})", user_id);
    
    // 2. 插入密码
    sqlx::query(
        r#"
        INSERT INTO user_password (user_id, password, create_at, update_at, deleted)
        VALUES (?, md5('123456'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
        "#
    )
    .bind(user_id)
    .execute(&mut *tx).await?;
    println!("✓ 插入密码");
    
    // 3. 插入邮箱
    sqlx::query(
        r#"
        INSERT INTO user_email (user_id, email, create_at, update_at, deleted)
        VALUES (?, 'yehunhk@163.com', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
        "#
    )
    .bind(user_id)
    .execute(&mut *tx).await?;
    println!("✓ 插入邮箱: yehunhk@163.com");
    
    // 4. 插入手机
    sqlx::query(
        r#"
        INSERT INTO user_phone (user_id, phone, create_at, update_at, deleted)
        VALUES (?, '13800000000', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
        "#
    )
    .bind(user_id)
    .execute(&mut *tx).await?;
    println!("✓ 插入手机: 13800000000");
    
    // 提交事务
    tx.commit().await?;
    
    println!("✅ 初始数据插入成功！");
    println!();
    println!("📋 初始账户信息:");
    println!("   用户名: yehun");
    println!("   密码: 123456");
    println!("   邮箱: yehunhk@163.com");
    println!("   手机: 13800000000");
    println!("   类型: 会员 (Member)");
    println!();
    println!("💡 提示: 可以使用此账户登录测试");
    println!();
    println!("🔐 MFA 多因素认证:");
    println!("   - 支持 TOTP (Google Authenticator, Authy 等)");
    println!("   - 支持备用码登录");
    println!("   - API: /api/mfa/*");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_init_database() {
        let db_path = "/tmp/test_auth_sphere.db";
        
        // 如果文件存在，先删除
        if std::path::Path::new(db_path).exists() {
            std::fs::remove_file(db_path).unwrap();
        }
        
        let result = init_database(db_path).await;
        assert!(result.is_ok());
        
        // 验证文件已创建
        assert!(std::path::Path::new(db_path).exists());
        
        // 清理测试文件
        std::fs::remove_file(db_path).unwrap();
    }
}
