use std::io::Read;
use actix_web::{Error, HttpResponse, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;

const WEB_NOT_BUILD: &'static str = r#"
<!DOCTYPE html>
<html>
<head><meta charset="UTF-8"><title>Auth-Sphere</title></head>
<body style="font-family: Arial, sans-serif; padding: 40px; text-align: center;">
    <h1>🚀 Auth-Sphere API Server</h1>
    <p>后端服务运行正常</p>
    <hr>
    <p style="color: #666;">
        前端未编译或未部署<br>
        <small>请运行: cd web && npm run build</small>
    </p>
    <hr>
    <p style="font-size: 12px; color: #999;">
        API 文档请参考项目 README
    </p>
</body>
</html>
"#;

// 前端静态文件目录（编译后的 Vue 应用）
fn get_frontend_dist_dir() -> String {
    env!("CARGO_MANIFEST_DIR") 
        .replace("crates/api", "web/dist")
}

/// 提供首页
pub async fn index() -> impl Responder {
    let dist_dir = get_frontend_dist_dir();
    let index_path = PathBuf::from(&dist_dir).join("index.html");
    let html = NamedFile::open(&index_path).map(|mut file| {
        let mut text = String::new();
        file.read_to_string(&mut text).map(|_n| text)
    }).flatten().unwrap_or(WEB_NOT_BUILD.to_string());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


pub async fn assets(req: actix_web::HttpRequest) -> Result<NamedFile, Error> {
    let filename = req.match_info().query("filename");
    let dist_dir = get_frontend_dist_dir();
    let file_path = PathBuf::from(&dist_dir)
        .join("assets")
        .join(filename);
    tracing::debug!("Serving file: {:?}", file_path);
    let file = NamedFile::open(file_path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(actix_web::http::header::ContentDisposition {
            disposition: actix_web::http::header::DispositionType::Attachment,
            parameters: vec![],
        }))
}

/// SPA 回退处理器 - 所有非 API 路由都返回 index.html
pub async fn spa_fallback() -> impl Responder {
    let dist_dir = get_frontend_dist_dir();
    let index_path = PathBuf::from(&dist_dir).join("index.html");
    let (
        mut builder,
        content_type,
        html
    ) = NamedFile::open(&index_path).map(|mut file| {
        let mut text = String::new();
        file.read_to_string(&mut text).map(|_n| (
            HttpResponse::Ok(),
            "text/html; charset=utf-8",
            text
        ))
    }).flatten().unwrap_or((
        HttpResponse::NotFound(),
        "text/plain; charset=utf-8",
        WEB_NOT_BUILD.to_string()
    ));
    builder.content_type(content_type).body(html)
}
