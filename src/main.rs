mod http_header;
mod http_request;
mod http_request_handler;
mod http_response;

use http_request::HttpRequest;
use itertools::Itertools;
use tokio::io::AsyncWriteExt;

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
    stream: tokio::net::TcpStream,
}

impl TcpStreamHandler {
    fn new(stream: tokio::net::TcpStream) -> Self {
        Self { stream }
    }

    async fn handle(&mut self) {
        let http_request = HttpRequest::from_tcp_stream(&mut self.stream).await;
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
            .await
            .expect("Can send response");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4221")
        .await
        .expect("Should be able to bind");

    loop {
        let connection = listener.accept().await;
        match connection {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_tcp_stream(stream).await;
                });
            }
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}

async fn handle_tcp_stream(stream: tokio::net::TcpStream) {
    let mut tcp_stream_handler = TcpStreamHandler::new(stream);
    tcp_stream_handler.handle().await;
}
