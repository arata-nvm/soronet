use soronet::{
    http::{request::HttpMethod, status},
    server::Server,
};

fn main() {
    let mut server = Server::new();
    server.add(HttpMethod::Get, "/", |ctx| ctx.string("hogehoge"));
    server.add(HttpMethod::Get, "/error", |ctx| {
        ctx.status(status::NOT_FOUND).string("not found")
    });
    server.listen("localhost:8080");
}
