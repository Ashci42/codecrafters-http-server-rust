use crate::{
    http_header::{ContentLength, ContentType, HttpHeader, UserAgent},
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
            HttpHeader::ContentLength(ContentLength::new(content_length)),
        ]),
        Some(echo),
    )
}

pub fn handle_not_found() -> HttpResponse {
    HttpResponse::new(HttpResponseCode::NotFound, None, None)
}

pub fn handle_user_agent(user_agent: &UserAgent) -> HttpResponse {
    let user_agent_value = user_agent.value().to_string();
    let content_length = user_agent_value.len();

    HttpResponse::new(
        HttpResponseCode::Ok,
        Some(vec![
            HttpHeader::ContentType(ContentType::TextPlain),
            HttpHeader::ContentLength(ContentLength::new(content_length)),
        ]),
        Some(user_agent_value),
    )
}
