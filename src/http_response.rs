use std::fmt::Display;

use crate::http_header::HttpHeader;

pub enum HttpResponseCode {
    Ok,
    NotFound,
    Created,
}

impl Display for HttpResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::Created => write!(f, "201 Created"),
        }
    }
}

pub struct HttpResponse {
    http_response_code: HttpResponseCode,
    http_headers: Option<Vec<HttpHeader>>,
    http_response_body: Option<String>,
}

impl HttpResponse {
    pub fn new(
        http_response_code: HttpResponseCode,
        http_headers: Option<Vec<HttpHeader>>,
        http_response_body: Option<String>,
    ) -> Self {
        Self {
            http_response_code,
            http_headers,
            http_response_body,
        }
    }

    fn status_line(&self) -> String {
        format!("HTTP/1.1 {}", self.http_response_code)
    }

    fn headers(&self) -> String {
        match &self.http_headers {
            Some(http_headers) => {
                let mut headers = String::new();
                for http_header in http_headers {
                    headers.push_str(&http_header.to_string());
                    headers.push_str("\r\n");
                }

                headers
            }
            None => String::from(""),
        }
    }

    fn response_body(&self) -> String {
        self.http_response_body
            .as_ref()
            .map_or(String::from(""), |http_response_body| {
                http_response_body.clone()
            })
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\r\n{}\r\n{}",
            self.status_line(),
            self.headers(),
            self.response_body()
        )
    }
}
