use std::{error::Error, fmt::Display};

pub enum HttpHeader {
    ContentType(ContentType),
    ContentLength(ContentLength),
    UserAgent(UserAgent),
    Host(Host),
    Accept(Accept),
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeader::ContentType(content_type) => write!(f, "Content-Type: {}", content_type),
            HttpHeader::ContentLength(content_length) => {
                write!(f, "Content-Length: {}", content_length)
            }
            HttpHeader::UserAgent(user_agent) => write!(f, "User-Agent: {}", user_agent),
            HttpHeader::Host(host) => write!(f, "Host: {}", host),
            HttpHeader::Accept(accept) => write!(f, "Accept: {}", accept),
        }
    }
}

impl TryFrom<&str> for HttpHeader {
    type Error = HttpHeaderError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let http_header_parts = value.split_once(": ");

        if let Some(http_header_parts) = http_header_parts {
            match http_header_parts {
                ("Host", host) => Ok(HttpHeader::Host(Host::new(host.to_string()))),
                ("User-Agent", user_agent) => Ok(HttpHeader::UserAgent(UserAgent::new(
                    user_agent.to_string(),
                ))),
                ("Accept", accept) => Ok(HttpHeader::Accept(Accept::new(accept.to_string()))),
                _ => Err(HttpHeaderError::ParseString(value.to_string())),
            }
        } else {
            Err(HttpHeaderError::ParseString(value.to_string()))
        }
    }
}

pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
            ContentType::ApplicationOctetStream => write!(f, "application/octet-stream"),
        }
    }
}

#[derive(Debug)]
pub enum HttpHeaderError {
    ParseString(String),
}

impl Display for HttpHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeaderError::ParseString(s) => {
                write!(f, "Failed to parse header from string: {}", s)
            }
        }
    }
}

impl Error for HttpHeaderError {}

pub struct ContentLength(usize);

impl ContentLength {
    pub fn new(content_length: usize) -> Self {
        Self(content_length)
    }
}

impl Display for ContentLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct UserAgent(String);

impl UserAgent {
    pub fn value(&self) -> &str {
        &self.0
    }

    fn new(user_agent: String) -> Self {
        Self(user_agent)
    }
}

impl Display for UserAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Host(String);

impl Host {
    fn new(host: String) -> Self {
        Self(host)
    }
}

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Accept(String);

impl Accept {
    pub fn new(accept: String) -> Self {
        Self(accept)
    }
}

impl Display for Accept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
