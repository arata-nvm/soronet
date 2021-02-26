pub mod context;
pub mod handler;

use std::net::{TcpListener, TcpStream};

use log::info;

use crate::{
    http::{
        request::{self, HttpMethod, HttpRequest},
        response::HttpResponse,
        status, HttpHeaders,
    },
    server::context::Context,
};

use self::handler::{Route, RouteHandler, StaticFileHandler};

pub struct Server {
    routes: Vec<Route>,
}

impl Server {
    pub fn new() -> Self {
        env_logger::init();
        Self { routes: Vec::new() }
    }

    pub fn listen(&mut self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {
            self.handle_connection(&mut stream.unwrap());
        }
    }

    fn handle_connection(&mut self, stream: &mut TcpStream) {
        let req = request::parse(stream);
        info!(
            "{:?} {} from {}",
            req.method,
            req.target,
            stream.peer_addr().unwrap()
        );

        for route in &self.routes {
            if route.method == (&req).method && route.path == (&req).target {
                let ctx = Context::new(req);
                let response = route.h.handle(ctx);
                response.write_to(stream).unwrap();
                return;
            }
        }

        self.not_found(req).write_to(stream).unwrap();
    }

    fn not_found(&mut self, req: HttpRequest) -> HttpResponse {
        HttpResponse {
            version: req.version,
            status_code: status::NOT_FOUND,
            headers: HttpHeaders::new(),
            body: None,
        }
    }

    pub fn add<H: RouteHandler + 'static>(&mut self, method: HttpMethod, path: &str, h: H) {
        self.routes.push(Route {
            method,
            path: path.into(),
            h: Box::new(h),
        });
    }

    pub fn static_file(&mut self, path: &str, file_path: &str) {
        self.add(HttpMethod::Get, path, StaticFileHandler::new(file_path));
    }
}
