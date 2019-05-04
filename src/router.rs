use crate::response::Response;
use crate::request::Request;

use crate::request::method as Method;

pub type Handler = fn(Request) -> Response;

pub struct Route {
  pub  pattern: String,
  pub  method: String,
  pub  callback: Handler,
}


pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Vec::new(),
        }
    }

    fn route(&mut self, method: &str, pattern: &str, f: Handler) {
        let r = Route{method: String::from(method), pattern: String::from(pattern), callback: f} ;
        self.routes.push(r);
    }

    pub fn get(&mut self, pattern: &str, f: Handler) {
        self.route(Method::GET, pattern, f)
    }

    pub fn post(&mut self, pattern: &str, f: Handler)  {
        self.route(Method::POST, pattern, f)
    }

}