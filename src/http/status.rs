use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct HttpStatusCode(u16);

impl HttpStatusCode {
    pub fn canonical_reason(&self) -> &str {
        match self {
            &OK => "Ok",
            &MOVED_PERMANENTLY => "Moved Permanently",
            &NOT_FOUND => "Not Found",
            _ => panic!(),
        }
    }
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub const OK: HttpStatusCode = HttpStatusCode(200);

pub const MOVED_PERMANENTLY: HttpStatusCode = HttpStatusCode(301);

pub const NOT_FOUND: HttpStatusCode = HttpStatusCode(404);
