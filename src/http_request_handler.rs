use std::path::PathBuf;

use crate::{
    http_header::{ContentLength, ContentType, HttpHeader, UserAgent},
    http_response::{HttpResponse, HttpResponseCode},
};

pub fn handle_get_root() -> HttpResponse {
    HttpResponse::new(HttpResponseCode::Ok, None, None)
}

pub fn handle_get_echo(echo: String) -> HttpResponse {
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

pub fn handle_get_user_agent(user_agent: &UserAgent) -> HttpResponse {
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

pub async fn handle_get_files(file: String, directory: &PathBuf) -> HttpResponse {
    let mut file_path = PathBuf::from(directory);
    file_path.push(file);

    if file_path.exists() {
        let file_contents = tokio::fs::read_to_string(file_path)
            .await
            .expect("Can read file contents");
        let content_length = file_contents.len();

        HttpResponse::new(
            HttpResponseCode::Ok,
            Some(vec![
                HttpHeader::ContentType(ContentType::ApplicationOctetStream),
                HttpHeader::ContentLength(ContentLength::new(content_length)),
            ]),
            Some(file_contents),
        )
    } else {
        HttpResponse::new(HttpResponseCode::NotFound, None, None)
    }
}

pub async fn handle_post_files(file: String, directory: &PathBuf, text: String) -> HttpResponse {
    let mut file_path = PathBuf::from(directory);
    file_path.push(file);
    tokio::fs::write(file_path, text)
        .await
        .expect("Can write new file");

    HttpResponse::new(HttpResponseCode::Created, None, None)
}
