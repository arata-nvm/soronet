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

    server.add(
        HttpMethod::Get,
        "/users/{group}/{id}",
        FunctionHandler::new(|ctx| {
            let group = ctx.params.get("group").cloned().unwrap();
            let id = ctx.params.get("id").cloned().unwrap();
            ctx.string(&format!("group: {}\nid: {}", group, id))
        }),
    );

    server.static_file("/static/index.html", "./public/index.html");
    server.static_file("/static/404.html", "./public/404.html");

    server.listen("localhost:8080");
}
