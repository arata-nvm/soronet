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
    server::{
        context::Context,
        handler::{Handler, HandlerFunction},
    },
};

pub struct Server {
    handlers: Vec<Handler>,
}

impl Server {
    pub fn new() -> Self {
        env_logger::init();
        Self {
            handlers: Vec::new(),
        }
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

        for handler in &self.handlers {
            if handler.method == (&req).method && handler.path == (&req).target {
                let ctx = Context::new(req);
                let response = (handler.f)(ctx);
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

    pub fn add(&mut self, method: HttpMethod, path: &str, f: HandlerFunction) {
        self.handlers.push(Handler {
            method,
            path: path.into(),
            f,
        });
    }
}
