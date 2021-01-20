use std::net::{ TcpListener, TcpStream };
use std::io::prelude::*;
use regex::Regex;
use crossbeam_utils::thread;
use std::collections::HashMap;
use scoped_threadpool::Pool;
use lazy_static;
use std::borrow::Cow;
// use crate::pool::ThreadPool;
use crate::request::Request;
use crate::response::Response;
use crate::serveroptions::ServerOptions;
use crate::statuscode::StatusCode;

#[derive(Clone, Debug)]
pub struct Server<'a> {
    options: &'a ServerOptions<'a>,
    pub heartbeats: usize,
    router: Router<'a>,
}

#[derive(Clone, Debug)]
pub struct Router<'a> {
    strict_slash: bool,
    pub routes: HashMap<String, &'a Route<'a>>,
}

impl<'a> Router<'a> {

    pub fn new() -> Router<'a> {
        Router {
            strict_slash: true,
            routes: HashMap::new(),
        }
    }

    fn map_route(&mut self, route: &'a Route<'a>) {
        self.routes.insert(route.http_method.clone().to_owned() + route.path, route);
    }

}

#[derive(Clone, Debug)]
pub struct Route<'a> {
    pub handler: fn(Request, Response),
    pub path: &'a str,
    pub http_method: &'a str,
}

impl<'a> Route<'a> {

    pub fn new(path: &'a str, http_method: &'a str, handler: fn(Request, Response)) -> Route<'a> {
        Route {
            path: path,
            http_method: http_method,
            handler: handler,
        }
    }

}

lazy_static! {
    static ref REQUEST_HEADER_REGEX: Regex = Regex::new(r"^(\w+) (\S+) HTTP/1.1").unwrap();
    static ref HOST_REGEX: Regex = Regex::new(r"Host: (\S+)").unwrap();
    static ref CONTENT_TYPE_REGEX: Regex = Regex::new(r"Content-Type: (\S+)").unwrap();
    static ref CONTENT_LENGTH_REGEX: Regex = Regex::new(r"(?i)Content-Length: (\d+)").unwrap();
    static ref CONTENT_REGEX: Regex = Regex::new(r"Connection: (\S+)").unwrap();
    static ref USER_AGENT_REGEX: Regex = Regex::new(r"User-Agent: (\S+)").unwrap();
}

impl<'a> Server<'a> {

    pub fn new(router: Router<'a>) -> Server<'a> {
        Server {
            options: &ServerOptions::new(),
            heartbeats: 0,
            router: router,
        }
    }

    pub fn get_listener(&self) -> TcpListener {
        TcpListener::bind((self.options.host, self.options.port as u16)).unwrap()
    }

    pub fn map_route(&mut self, route: &'a Route<'a>) {
        self.router.map_route(route);
    }

    pub fn handle_connection(&self, stream:  &mut TcpStream) -> std::io::Result<(Request, Response)> {
        let mut byte_buffer = [0; 2048];
        stream.read(&mut byte_buffer).unwrap();
        //TODO: fix lifetime issue here rustc --explain E0716
        let buffer: &str = &String::from_utf8_lossy(&byte_buffer[..]).into_owned();
        assert!(REQUEST_HEADER_REGEX.is_match(buffer)); 

        let request_method;
        match REQUEST_HEADER_REGEX.captures(buffer) {
            Some(captures) => {
                request_method = captures.get(1).unwrap().as_str();
            },
            None => {
                request_method = "";
            }
        }

        let request_path;
        match REQUEST_HEADER_REGEX.captures(buffer) {
            Some(captures) => {
                request_path = captures.get(2).unwrap().as_str();
            },
            None => {
                request_path = "";
            }
        }

        let host;
        match HOST_REGEX.captures(buffer) {
            Some(captures) => {
                host = captures.get(1).unwrap().as_str();
            },
            None => {
                host = "";
            }
        }

        let content_type;
        match CONTENT_TYPE_REGEX.captures(buffer) {
            Some(captures) => {
                content_type = captures.get(1).unwrap().as_str();
            },
            None => {
                content_type = "";
            }
        }

        let content_length;
        match CONTENT_LENGTH_REGEX.captures(buffer) {
            Some(captures) => {
                content_length = captures.get(1).unwrap().as_str();
            },
            None => {
                content_length = "";
            }
        }

        let user_agent;
        match USER_AGENT_REGEX.captures(buffer) {
            Some(captures) => {
                user_agent = captures.get(1).unwrap().as_str();
            },
            None => {
                user_agent = "";
            }
        }

        let mut response = Response::new(stream.try_clone()?);
        if content_length != "" {
            response.content_length = content_length.parse::<usize>().unwrap();
        }

        let mut request = Request::new();
        request.host = host;
        request.content_type = content_type;
        request.user_agent = user_agent;
        request.request_method = request_method;
        request.path = request_path;
        
        let mut _content: &'a str;
        println!("{:?}", content_length);
        if content_length != "" {
            if let Some(_content) = CONTENT_REGEX.captures(buffer) {
                //Bug is over here
                request.body = &buffer[_content.get(1).unwrap().end() + 4 .. _content.get(1).unwrap().end() + 4 + content_length.parse::<usize>().unwrap()];
            }
        }
        Ok((request, response))
    }

    pub fn heartbeat(&mut self) -> &mut Self {
        self.heartbeats += 1;
        println!("{}", self.heartbeats);
        self
    }

    pub fn start(&self) {
        let tcp_listener: TcpListener = self.get_listener();
        println!("Running");
        println!("{:?}", tcp_listener);
        let mut pool = Pool::new(4);
        for stream in tcp_listener.incoming() {
            pool.scoped(| scope | {
                scope.execute(move || {
                    let (req, mut res) = self.handle_connection(&mut stream.unwrap()).unwrap();
                    let result = self.router.routes.get(&(req.request_method.clone().to_owned() + req.path));
                    match result {
                        Some(route) => {
                            ( route.handler )( req , res );
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


