use soronet::{
    http::{request::HttpMethod, status},
    server::{handler::FunctionHandler, Server},
};

fn main() {
    let mut server = Server::new();

    server.add(
        HttpMethod::Get,
        "/",
        FunctionHandler::new(|ctx| ctx.string("hogehoge")),
    );
    server.add(
        HttpMethod::Get,
        "/error",
        FunctionHandler::new(|ctx| ctx.status(status::NOT_FOUND).string("not found")),
    );

    server.static_file("/static/index.html", "./public/index.html");
    server.static_file("/static/404.html", "./public/404.html");

    server.listen("localhost:8080");
}
