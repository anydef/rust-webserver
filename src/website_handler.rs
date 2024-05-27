use std::fs;

use crate::http::method::Method;
use crate::http::StatusCode::HTTP_Ok;
use crate::http::StatusCode::NotFound;

use super::http::{Request, Response};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path= format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
              if path.starts_with(&self.public_path) {
                  fs::read_to_string(path).ok()
              } else { 
                  println!("Directory traversal attack attempted: {}", file_path);
                  None
              }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(HTTP_Ok, self.read_file("index.html")),
                "/hello" => Response::new(HTTP_Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(HTTP_Ok, Some(contents)),
                    None => Response::new(
                        NotFound, None),
                }
            }
            _ => Response::new(NotFound, None),

        }
    }
}