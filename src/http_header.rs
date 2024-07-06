use std::fmt::Display;

pub enum HttpHeader {
    ContentType(ContentType),
    ContentLength(usize),
}

impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpHeader::ContentType(content_type) => write!(f, "Content-Type: {}", content_type),
            HttpHeader::ContentLength(content_length) => {
                write!(f, "Content-Length: {}", content_length)
            }
        }
    }
}

pub enum ContentType {
    TextPlain,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
        }
    }
}
