use crate::{
    http::{request::HttpMethod, response::HttpResponse},
    server::context::Context,
};

pub struct Handler {
    pub method: HttpMethod,
    pub path: String,
    pub f: HandlerFunction,
}

pub type HandlerFunction = fn(Context) -> HttpResponse;
