pub mod context;
pub mod handler;

use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use log::info;
use regex::bytes::Regex;

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

        let best_route_index = self
            .routes
            .iter()
            .filter(|route| route.method == ((&req).method))
            .enumerate()
            .filter_map(|(i, route)| {
                Some((i, route.path.find((&req).target.as_bytes())?.range().len()))
            })
            .max_by_key(|(_, len)| *len);

        let best_route = if let Some((index, _)) = best_route_index {
            self.routes.get(index).unwrap()
        } else {
            self.not_found(req).write_to(stream).unwrap();
            return;
        };
        info!("handled by {}", best_route.path);

        let captures = best_route.path.captures((&req).target.as_bytes());
        let params: HashMap<String, String> = best_route
            .path
            .capture_names()
            .flatten()
            .filter_map(|name| {
                Some((
                    name.to_string(),
                    String::from_utf8_lossy(captures.as_ref()?.name(name)?.as_bytes()).to_string(),
                ))
            })
            .collect();

        let mut ctx = Context::new(req);
        ctx.params.extend(params);
        let response = best_route.handle(ctx);
        response.write_to(stream).unwrap();
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
        let param_regex = Regex::new(r"\{(?P<n>[^/]+)\}").unwrap();
        let path = param_regex.replace_all(path.as_bytes(), r"(?P<$n>\w+)".as_bytes());
        let path = format!("^{}$", String::from_utf8_lossy(path.as_ref()));

        self.routes.push(Route {
            method,
            path: Regex::new(&path).unwrap(),
            h: Box::new(h),
        });
    }

    pub fn static_file(&mut self, path: &str, file_path: &str) {
        self.add(HttpMethod::Get, path, StaticFileHandler::new(file_path));
    }
}
