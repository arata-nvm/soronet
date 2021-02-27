use soronet::{
    http::{request::HttpMethod, response::HttpResponse, status},
    server::{handler::FunctionHandler, Server},
};

fn main() {
    let mut server = Server::new();

    server.add(
        HttpMethod::Get,
        "/",
        FunctionHandler::new(|_| HttpResponse::new().string("hogehoge")),
    );

    server.add(
        HttpMethod::Get,
        "/error",
        FunctionHandler::new(|_| {
            HttpResponse::new()
                .status(status::NOT_FOUND)
                .string("not found")
        }),
    );

    server.add(
        HttpMethod::Get,
        "/moved",
        FunctionHandler::new(|_| {
            HttpResponse::new()
                .status(status::MOVED_PERMANENTLY)
                .header("Location", "/")
        }),
    );

    server.add(
        HttpMethod::Get,
        "/users/{group}/{id}",
        FunctionHandler::new(|ctx| {
            let group = ctx.params.get("group").cloned().unwrap();
            let id = ctx.params.get("id").cloned().unwrap();
            HttpResponse::new().string(&format!("group: {}\nid: {}", group, id))
        }),
    );

    server.static_dir("/static", "./public");

    server.listen("localhost:8080");
}
