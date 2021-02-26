use std::{fs::File, io::Read};

use crate::http::{
    request::HttpRequest,
    response::HttpResponse,
    status::{self, HttpStatusCode},
    HttpHeaders,
};

pub struct Context {
    pub request: HttpRequest,
    pub response: HttpResponse,
}

impl Context {
    pub fn new(request: HttpRequest) -> Self {
        Self {
            response: HttpResponse {
                version: request.version.clone(),
                status_code: status::OK,
                headers: HttpHeaders::new(),
                body: None,
            },
            request,
        }
    }

    pub fn status(mut self, status_code: HttpStatusCode) -> Self {
        self.response.status_code = status_code;
        self
    }

    pub fn string(mut self, body: &str) -> HttpResponse {
        self.response.body = Some(body.into());
        self.response
    }

    pub fn file(self, file_path: &str) -> HttpResponse {
        let mut file = File::open(file_path).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        self.string(&s)
    }
}
