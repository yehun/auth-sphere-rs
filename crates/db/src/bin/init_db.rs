use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 从环境变量或参数获取数据库路径
    let db_path = env::args()
        .nth(1)
        .or_else(|| env::var("DATABASE_URL").ok())
        .unwrap_or_else(|| "/home/yehun/mystery.db".to_string());
    
    println!("====================================");
    println!("  Auth-Sphere 数据库初始化工具");
    println!("====================================");
    println!();
    
    // 执行初始化
    match auth_sphere_db::init_database(&db_path).await {
        Ok(_) => {
            println!();
            println!("🎉 数据库初始化成功！");
            println!("📁 数据库路径: {}", db_path);
            println!();
            println!("可用的表:");
            println!("  - user (用户基本信息)");
            println!("  - user_password (用户密码)");
            println!("  - user_email (用户邮箱)");
            println!("  - user_phone (用户手机)");
            println!();
            println!("下一步:");
            println!("  1. 启动 API 服务器: cargo run --bin auth-sphere-api");
            println!("  2. 访问 http://localhost:8080 查看演示页面");
        }
        Err(e) => {
            eprintln!("❌ 数据库初始化失败: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
