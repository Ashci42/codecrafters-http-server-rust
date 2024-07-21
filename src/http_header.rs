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
                ("Content-Type", content_type) => {
                    let content_type = ContentType::try_from(content_type)?;

                    Ok(HttpHeader::ContentType(content_type))
                }
                ("Content-Length", content_length) => {
                    let content_length = ContentLength::try_from(content_length)?;

                    Ok(HttpHeader::ContentLength(content_length))
                }
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

impl TryFrom<&str> for ContentType {
    type Error = ContentTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "text/plain" => Ok(Self::TextPlain),
            "application/octet-stream" => Ok(Self::ApplicationOctetStream),
            _ => Err(ContentTypeError::new(value)),
        }
    }
}

#[derive(Debug)]
pub enum HttpHeaderError {
    ParseString(String),
    InvalidContentType(ContentTypeError),
    InvalidContentLength(ContentLengthError),
}

impl Display for HttpHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseString(s) => write!(f, "Failed to parse header from string: {}", s),
            Self::InvalidContentType(content_type_err) => write!(f, "{}", content_type_err),
            Self::InvalidContentLength(content_length_err) => write!(f, "{}", content_length_err),
        }
    }
}

impl Error for HttpHeaderError {}

impl From<ContentTypeError> for HttpHeaderError {
    fn from(value: ContentTypeError) -> Self {
        Self::InvalidContentType(value)
    }
}

impl From<ContentLengthError> for HttpHeaderError {
    fn from(value: ContentLengthError) -> Self {
        Self::InvalidContentLength(value)
    }
}

pub struct ContentLength(usize);

impl ContentLength {
    pub fn new(content_length: usize) -> Self {
        Self(content_length)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

impl Display for ContentLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for ContentLength {
    type Error = ContentLengthError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let content_length = value.parse();

        match content_length {
            Ok(content_length) => Ok(Self::new(content_length)),
            Err(_) => Err(ContentLengthError::new(value)),
        }
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

#[derive(Debug)]
pub struct ContentTypeError {
    content_type: String,
}

impl ContentTypeError {
    fn new(content_type: &str) -> Self {
        Self {
            content_type: content_type.to_string(),
        }
    }
}

impl Display for ContentTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid content type: {}", self.content_type)
    }
}

impl Error for ContentTypeError {}

#[derive(Debug)]
pub struct ContentLengthError {
    content_length: String,
}

impl ContentLengthError {
    fn new(content_type: &str) -> Self {
        Self {
            content_length: content_type.to_string(),
        }
    }
}

impl Display for ContentLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid content type: {}", self.content_length)
    }
}

impl Error for ContentLengthError {}
