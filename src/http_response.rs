use std::fmt::Display;

pub enum HttpResponseCode {
    Ok,
    NotFound,
}

impl Display for HttpResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::NotFound => write!(f, "404 Not Found"),
        }
    }
}

pub struct HttpResponse {
    http_response_code: HttpResponseCode,
}

impl HttpResponse {
    pub fn new(http_response_code: HttpResponseCode) -> Self {
        Self { http_response_code }
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let http_response_code = self.http_response_code.to_string();
        write!(f, "HTTP/1.1 {}\r\n\r\n", http_response_code)
    }
}
