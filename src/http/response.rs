use std::{
    fs::File,
    io::{self, Read, Write},
};

use super::{
    status::{self, HttpStatusCode},
    HttpHeaders,
};

#[derive(Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status_code: HttpStatusCode,
    pub headers: HttpHeaders,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: status::OK,
            headers: HttpHeaders::new(),
            body: None,
        }
    }

    pub fn status(mut self, status_code: HttpStatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn string(mut self, body: &str) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn file(self, file_path: &str) -> HttpResponse {
        let contents = File::open(file_path).map(|mut file| {
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            s
        });

        match contents {
            Ok(contents) => self.string(&contents),
            Err(_) => self.status(status::NOT_FOUND),
        }
    }

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
