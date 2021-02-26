use std::io::{self, Write};

use super::{status::HttpStatusCode, HttpHeaders};

#[derive(Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: HttpStatusCode,
    pub headers: HttpHeaders,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn write_to<W: Write>(&self, w: &mut W) -> io::Result<()> {
        // status-line
        write!(
            w,
            "{} {} {}\r\n",
            self.version,
            self.status_code,
            self.status_code.canonical_reason()
        )?;

        // header-field
        for (k, v) in &self.headers {
            write!(w, "{}: {}\r\n", k, v)?;
        }

        write!(w, "\r\n")?;
        if let Some(ref body) = self.body {
            write!(w, "{}", body)?;
        }

        Ok(())
    }
}
