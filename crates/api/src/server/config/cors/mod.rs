use actix_cors::Cors;

pub fn get_config() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method()
    // Cors::default()
    //     .allowed_origin("https://www.rust-lang.org")
    //     .allowed_origin_fn(|origin, _req_head| {
    //         origin.as_bytes().ends_with(b".rust-lang.org")
    //     })
    //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    //     .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    //     .allowed_header(http::header::CONTENT_TYPE)
    //     .max_age(3600)
}
