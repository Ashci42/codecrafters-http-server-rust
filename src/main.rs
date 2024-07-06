mod http_header;
mod http_request;
mod http_request_handler;
mod http_response;

use std::io::Write;

use http_request::HttpRequest;
use itertools::Itertools;

enum HttpRequestType {
    Root,
    Echo(String),
    UserAgent,
}

impl HttpRequestType {
    fn try_new(request_target: &str) -> Option<Self> {
        let request_target_parts = request_target.split('/').collect_vec();
        match request_target_parts[..] {
            ["", ""] => Some(HttpRequestType::Root),
            ["", "echo", echo] => Some(HttpRequestType::Echo(echo.to_string())),
            ["", "user-agent"] => Some(HttpRequestType::UserAgent),
            _ => None,
        }
    }
}

struct TcpStreamHandler {
    stream: std::net::TcpStream,
}

impl TcpStreamHandler {
    fn new(stream: std::net::TcpStream) -> Self {
        Self { stream }
    }

    fn handle(&mut self) {
        let http_request = HttpRequest::from(&self.stream);
        let request_target = http_request.request_target();
        let request_type = HttpRequestType::try_new(request_target);
        let http_response = match request_type {
            Some(HttpRequestType::Root) => http_request_handler::handle_root(),
            Some(HttpRequestType::Echo(echo)) => http_request_handler::handle_echo(echo),
            Some(HttpRequestType::UserAgent) => http_request_handler::handle_user_agent(
                http_request
                    .user_agent()
                    .expect("Request should have an user agent"),
            ),
            None => http_request_handler::handle_not_found(),
        };
        let http_response = http_response.to_string();
        self.stream
            .write_all(http_response.as_bytes())
            .expect("Can send response");
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:4221").expect("Should be able to bind");

    for stream in listener.incoming() {
        handle_connection(stream);
    }
}

fn handle_connection(stream: std::io::Result<std::net::TcpStream>) {
    match stream {
        Ok(stream) => {
            handle_tcp_stream(stream);
        }
        Err(e) => {
            println!("error: {e}");
        }
    }
}

fn handle_tcp_stream(stream: std::net::TcpStream) {
    let mut tcp_stream_handler = TcpStreamHandler::new(stream);
    tcp_stream_handler.handle();
}
