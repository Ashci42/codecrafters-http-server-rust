use std::io::BufRead;

pub struct HttpRequest {
    request_line: RequestLine,
}

impl HttpRequest {
    pub fn request_target(&self) -> &str {
        &self.request_line.request_target
    }
}

impl From<&std::net::TcpStream> for HttpRequest {
    fn from(value: &std::net::TcpStream) -> Self {
        let mut buf_reader = std::io::BufReader::new(value);
        let mut request_line = String::new();
        buf_reader
            .read_line(&mut request_line)
            .expect("Can read request line from tcp stream");
        let request_line = RequestLine::from(request_line);

        Self { request_line }
    }
}

struct RequestLine {
    request_target: String,
}

impl From<String> for RequestLine {
    fn from(value: String) -> Self {
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
