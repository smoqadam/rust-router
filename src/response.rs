use std::io::{Write};
use std::net::{TcpStream};


pub struct Response {
    pub status: i32,
    pub content_type: String,
    pub body: String,
}

impl Response {
    pub fn new(body: String, content_type: String, status: i32) -> Response {
        Response {
            status: status,
            content_type: content_type,
            body: body,
        }
    }

    pub fn html(body: String, status: i32) -> Response {
        Response::new(body, String::from("Content-Type: text/html"), status)
    }


    pub fn json(body: String, status: i32) -> Response {
        Response::new(body, String::from("Content-Type: application/json"), status)
    }

    pub fn result(&self, stream: &mut TcpStream) {
        let result = format!(
            "{} {} {}\n{}\n\n{}",
            "HTTP/1.1", self.status, "OK", self.content_type, self.body
        );
        println!("{}", result);
        let _ = stream.write_all(result.as_bytes());
    }
}
