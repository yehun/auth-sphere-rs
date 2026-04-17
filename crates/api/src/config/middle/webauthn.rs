use webauthn_rs::prelude::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};
use crate::config::application;

pub fn init() -> Webauthn {
    let config = application::get();
    let port = config.server.port.unwrap_or(8000);
    let rp_id = config.server.server;
    let rp_origin_str = format!("http://{}:{}", rp_id, port);
    let rp_origin = Url::parse(&rp_origin_str).expect("Invalid origin");
    let builder = WebauthnBuilder::new(&rp_id, &rp_origin).expect("Invalid webauthn");
    builder.build().expect("Invalid webauthn")
}