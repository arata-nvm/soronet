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
        HttpResponse::new().file(&self.file_path)
    }
}

pub struct StaticDirHandler {
    pub path: String,
    pub dir_path: String,
}

impl StaticDirHandler {
    pub fn new(path: &str, dir_path: &str) -> Self {
        Self {
            path: path.into(),
            dir_path: dir_path.into(),
        }
    }
}

impl RouteHandler for StaticDirHandler {
    fn handle(&self, ctx: Context) -> HttpResponse {
        let file_path = ctx.request.target.replace(&self.path, &self.dir_path);
        HttpResponse::new().file(&file_path)
    }
}
