use actix_http::h1;
use actix_http::header::HeaderMap;
use actix_web::{dev, web};
use std::collections::HashMap;

pub trait ToMap {
    fn to_map(&self) -> HashMap<String, String>;
}

impl ToMap for HeaderMap {
    fn to_map(&self) -> HashMap<String, String> {
        let mut header_map: HashMap<String, String> = HashMap::new();
        self.iter().for_each(|x| {
            header_map.insert(x.0.to_string(), x.1.to_str().unwrap().to_string());
        });
        // serde_json::to_string(&header_map).unwrap()
        header_map
    }
}

pub fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut payload) = h1::Payload::create(true);
    payload.unread_data(buf);
    dev::Payload::from(payload)
}
