use std::net::{ TcpListener, TcpStream };
use std::io::prelude::*;
// use std::io::{ BufReader, BufRead, BufWriter, Error, ErrorKind };
// use std::thread;
use regex::Regex;
use crossbeam_utils::thread;
use std::collections::HashMap;

use crate::request::Request;
use crate::response::Response;
use crate::serveroptions::ServerOptions;
use crate::statuscode::StatusCode;

#[derive(Clone, Debug)]
pub struct Server {
    pub options: &'static ServerOptions,
    pub heartbeats: usize,
    pub router: Router,
}

#[derive(Clone, Debug)]
pub struct Router {
    pub strict_slash: bool,
    pub routes: HashMap<String, Route>,
}

impl Router{

    pub fn new() -> Router {
        Router {
            strict_slash: true,
            routes: HashMap::new(),
        }
    }

    fn map_route(&mut self, route: Route) {
        self.routes.insert(route.http_method.clone().to_owned() + route.path, route);
    }

}

#[derive(Clone, Debug)]
pub struct Route {
    //Route path
    pub handler: fn(Request, Response),
    pub path: &'static str,
    //Route request type(GET, POST, UPDATE...)
    pub http_method: &'static str,
}


impl Route {

    pub fn new(path: &'static str, http_method: &'static str, handler: fn(Request, Response)) -> Route {
        Route {
            path: path,
            http_method: http_method,
            handler: handler,
        }
    }

}

impl Server {

    pub fn new(router: Router) -> Server {
        Server {
            options: ServerOptions::new(),
            heartbeats: 0,
            router: router,
        }
    }

    pub fn get_listener(&self) -> Option<TcpListener> {
        let mut host: String = self.options.host.to_owned();
        host.push_str(":");
        host.push_str(&self.options.port.to_string()[..]);
        Some(TcpListener::bind(host).unwrap())
    }

    pub fn map_route(&mut self, route: Route) {
        self.router.map_route(route);
    }

    pub fn handle_connection(&self, mut  stream:  &TcpStream) -> std::io::Result<(Request, Response)> {
        let mut byte_buffer = [0; 2048];
        stream.read(&mut byte_buffer).unwrap();
        let buffer = String::from_utf8_lossy(&byte_buffer[..]).to_string();

        let mut request = Request::new();
        let request_header_regex = Regex::new(r"^(\w+) (\S+) HTTP/1.1").unwrap();
        let host_regex = Regex::new(r"Host: (\S+)").unwrap();
        let content_type_regex = Regex::new(r"Content-Type: (\S+)").unwrap();
        let content_length_regex = Regex::new(r"content-length: (\d+)").unwrap();
        let content_regex = Regex::new(r"Connection: (\S+)").unwrap();
        let user_agent_regex = Regex::new(r"User-Agent: (\S+)").unwrap();

        assert!(request_header_regex.is_match(&buffer.to_string())); 

        let bufferwith_static_lifetime: &'static str = Box::leak(buffer.into_boxed_str());

        let request_method;
        match request_header_regex.captures(bufferwith_static_lifetime) {
            Some(captures) => {
                request_method = captures.get(1).unwrap().as_str();
            },
            None => {
                request_method = "";
            }
        }

        let request_path;
        match request_header_regex.captures(bufferwith_static_lifetime) {
            Some(captures) => {
                request_path = captures.get(2).unwrap().as_str();
            },
            None => {
                request_path = "";
            }
        }

        let host;
        match host_regex.captures(bufferwith_static_lifetime) {
            Some(captures) => {
                host = captures.get(1).unwrap().as_str();
            },
            None => {
                host = "";
            }
        }

        let content_type;
        match content_type_regex.captures(bufferwith_static_lifetime) {
            Some(captures) => {
                content_type = captures.get(1).unwrap().as_str();
            },
            None => {
                content_type = "text/html";
            }
        }

        let content_length;
        match content_length_regex.captures(bufferwith_static_lifetime) {
            Some(captures) => {
                content_length = captures.get(1).unwrap().as_str();
            },
            None => {
                content_length = "0";
            }
        }

        let user_agent;
        match user_agent_regex.captures(bufferwith_static_lifetime) {
            Some(captures) => {
                user_agent = captures.get(1).unwrap().as_str();
            },
            None => {
                user_agent = "";
            }
        }

        let mut response = Response::new(stream.try_clone()?);
        response.content_length = content_length.parse::<usize>().unwrap();

        request.host = host;
        request.content_type = content_type;
        request.user_agent = user_agent;
        request.request_method = request_method;
        request.path = request_path;

        let mut _content: &'static str;
        if let Some(_content) = content_regex.captures(bufferwith_static_lifetime) {
            request.body = &bufferwith_static_lifetime[_content.get(1).unwrap().end() + 1.. _content.get(1).unwrap().end() + 1 + content_length.parse::<usize>().unwrap()];
        }

        Ok((request, response))
    }

    pub fn heartbeat(&mut self) -> &mut Server {
        self.heartbeats += 1;
        println!("{}", self.heartbeats);
        self
    }

    pub fn start(&mut self) {
        println!("Running");
        println!("{:?}", self.get_listener().as_ref().unwrap());
        for stream in self.get_listener().as_ref().unwrap().incoming() {
            thread::scope(|s| {
                s.spawn(|_| {
                    let (req, mut res) = self.handle_connection(&stream.unwrap()).unwrap();
                    let result = self.router.routes.get(&(req.request_method.clone().to_owned() + req.path));
                    match result {
                        Some(route) => {
                            (route.handler)(req, res);
                        }, 
                        _ => {
                            res.send_status(StatusCode::NotFound);
                        }
                    }
                });
            });
        }
    }

}


