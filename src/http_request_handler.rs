use crate::{
    http_header::{ContentType, HttpHeader},
    http_response::{HttpResponse, HttpResponseCode},
};

pub fn handle_root() -> HttpResponse {
    HttpResponse::new(HttpResponseCode::Ok, None, None)
}

pub fn handle_echo(echo: String) -> HttpResponse {
    let content_length = echo.len();

    HttpResponse::new(
        HttpResponseCode::Ok,
        Some(vec![
            HttpHeader::ContentType(ContentType::TextPlain),
            HttpHeader::ContentLength(content_length),
        ]),
        Some(echo),
    )
}

pub fn handle_not_found() -> HttpResponse {
    HttpResponse::new(HttpResponseCode::NotFound, None, None)
}
