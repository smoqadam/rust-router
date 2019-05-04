use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead};

pub mod method{
    pub const GET: &str = "GET";
    pub const POST: &str = "POST";
}

pub struct Request {
    path: String,
    method: String,
    params: HashMap<String, String>,
}

impl Request {
    pub fn parse(stream:&mut TcpStream) -> Request {

        let mut lines = String::new();
        let mut reader = BufReader::new(stream);
        let _ = reader.read_line(&mut lines);
        let mut line = lines.split_whitespace();
        
        let method = match line.nth(0) {
            Some(e) => e,
            None => "GET",
        };

        let path = match line.nth(0) {
            Some(e) => e,
            None => "/",
        };
        let parts = Request::parse_path(path);
        let req_path = parts[0];
        let query_string = match parts.get(1).or(None) {
            Some(q) => {
                let tags: HashMap<String, String> = q.split('&')
                    .map(|kv| kv.split('=').collect::<Vec<&str>>())
                    .map(|vec| {
                        assert_eq!(vec.len(), 2);
                        (vec[0].to_string(), vec[1].to_string())
                    })
                    .collect();
                    tags
            },

            None => HashMap::new(),
        };

        Request {
            method: method.to_string(),
            path: req_path.to_string(),
            params: query_string,
        }
        
    }

    fn parse_path(line: &str) -> Vec<&str> {
        let parts: Vec<&str> = line.split("?").collect();
        parts
    }

    pub fn get(&self, key: &str) -> &str {
        match self.params.get(key) {
            Some(e) => e,
            None => "",
        }
    }

    pub fn method(&self) -> String {
        self.method.to_string()
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }
}

