use std::net::{TcpListener, TcpStream};

use super::Router;
use crate::router::Handler;
use crate::request::Request;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(router: Router) -> Server {
        Server{ router: router}
    }

    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening to {}", addr);
        for stream in listener.incoming() {
            self.handle(&mut stream.unwrap());
        }
    }

    fn handle(&self, stream: &mut TcpStream) {
        let req = Request::parse(stream);
        for r in &self.router.routes {
            if r.method == req.method() && r.pattern == req.path() {
                self.dispatch(stream, r.callback, req);
                break;
            }
        }
    }

    fn dispatch(&self, stream:&mut TcpStream, handler: Handler, req: Request) {
        let response = (handler)(req);    
        response.result(stream);
    }
}