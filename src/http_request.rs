use std::slice::Iter;

use tokio::io::AsyncBufReadExt;

use crate::http_header::{HttpHeader, UserAgent};

pub struct HttpRequest {
    request_line: RequestLine,
    http_headers: HttpHeaders,
}

impl HttpRequest {
    pub fn request_target(&self) -> &str {
        &self.request_line.request_target
    }

    pub fn user_agent(&self) -> Option<&UserAgent> {
        for http_header in &self.http_headers {
            if let HttpHeader::UserAgent(user_agent) = http_header {
                return Some(user_agent);
            }
        }

        None
    }

    pub async fn from_tcp_stream(stream: &mut tokio::net::TcpStream) -> Self {
        let mut buf_reader = tokio::io::BufReader::new(stream);

        let mut request_line = String::new();
        buf_reader
            .read_line(&mut request_line)
            .await
            .expect("Can read request line from tcp stream");
        let request_line = request_line.trim_end();
        let request_line = RequestLine::from(request_line);

        let mut http_headers = HttpHeaders::new();
        let mut header = String::new();
        buf_reader
            .read_line(&mut header)
            .await
            .expect("Can read headers from tcp stream");
        while !header.trim_end().is_empty() {
            http_headers.add(
                HttpHeader::try_from(header.trim_end())
                    .expect("All headers received should be valid"),
            );
            header.clear();
            buf_reader
                .read_line(&mut header)
                .await
                .expect("Can read headers from tcp stream");
        }

        Self {
            request_line,
            http_headers,
        }
    }
}

struct RequestLine {
    request_target: String,
}

impl From<&str> for RequestLine {
    fn from(value: &str) -> Self {
        let mut request_line_parts = value.split(' ');
        request_line_parts
            .next()
            .expect("Request line should have a HTTP method");
        let request_target = request_line_parts
            .next()
            .expect("Request line should have a request target");
        let request_target = request_target.to_string();

        Self { request_target }
    }
}

struct HttpHeaders(Vec<HttpHeader>);

impl HttpHeaders {
    fn new() -> Self {
        Self(vec![])
    }

    fn add(&mut self, http_header: HttpHeader) {
        self.0.push(http_header);
    }
}

impl<'a> IntoIterator for &'a HttpHeaders {
    type Item = &'a HttpHeader;

    type IntoIter = Iter<'a, HttpHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
