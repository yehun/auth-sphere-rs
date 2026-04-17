use actix_web::{HttpRequest, HttpResponse, Responder};

use include_dir::{include_dir, Dir};
use mime_guess::from_path;

static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../web/dist");

struct EmbeddedFileResponder(&'static [u8], String);

impl Responder for EmbeddedFileResponder {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mime = from_path(&self.1).first_or_octet_stream();
        HttpResponse::Ok()
            .content_type(mime)
            .body(self.0)
    }
}

pub(crate) async fn index(req: HttpRequest) -> impl Responder {
    let path = "index.html";
    match ASSETS_DIR.get_file(path) {
        Some(file) => EmbeddedFileResponder(file.contents(), path.to_string()).respond_to(&req),
        None => HttpResponse::NotFound().respond_to(&req),
    }
}

pub(crate) async fn assets(req: HttpRequest) -> impl Responder {
    let filename = req.match_info().query("filename");
    let path = format!("assets/{}", filename);
    match ASSETS_DIR.get_file(&path) {
        Some(file) => EmbeddedFileResponder(file.contents(), path).respond_to(&req),
        None => HttpResponse::NotFound().respond_to(&req),
    }
}