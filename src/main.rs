mod http_request;
mod http_response;

use std::io::Write;

use http_request::HttpRequest;
use http_response::{HttpResponse, HttpResponseCode};

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
        let http_response = match request_target {
            "/" => HttpResponse::new(HttpResponseCode::Ok),
            _ => HttpResponse::new(HttpResponseCode::NotFound),
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
