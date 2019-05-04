mod router;
use router::Router;
mod response;
use response::Response;

mod server;
use server::Server;
mod request;

fn main() {
    let mut router = Router::new();
    router.get("/home", home);
    Server::new(router).run("127.0.0.1:8989");
}

fn home(req: request::Request) -> Response {
    let res = format!("Welcome Home: {}", req.get("name"));
    Response::html(String::from(res), 200)
}

