use regex::bytes::Regex;

use crate::{
    http::{request::HttpMethod, response::HttpResponse},
    server::context::Context,
};

pub struct Route {
    pub method: HttpMethod,
    pub path: Regex,
    pub h: Box<dyn RouteHandler>,
}

impl Route {
    pub fn handle(&self, ctx: Context) -> HttpResponse {
        self.h.handle(ctx)
    }
}

pub trait RouteHandler {
    fn handle(&self, ctx: Context) -> HttpResponse;
}

pub struct FunctionHandler {
    pub f: fn(Context) -> HttpResponse,
}

impl FunctionHandler {
    pub fn new(f: fn(Context) -> HttpResponse) -> Self {
        Self { f }
    }
}

impl RouteHandler for FunctionHandler {
    fn handle(&self, ctx: Context) -> HttpResponse {
        (self.f)(ctx)
    }
}

pub struct StaticFileHandler {
    pub file_path: String,
}

impl StaticFileHandler {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

impl RouteHandler for StaticFileHandler {
    fn handle(&self, ctx: Context) -> HttpResponse {
        ctx.file(&self.file_path)
    }
}
