use std::net::{ TcpListener, TcpStream };
use std::io::prelude::*;
use std::io::{ BufReader, BufRead, BufWriter, Error, ErrorKind };
// use std::thread;
use regex::Regex;
use crossbeam_utils::thread;
use std::fs;
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
    pub strictSlash: bool,
    pub routes: HashMap<String, Route>,
}

impl Router{

    pub fn new() -> Router {
        Router {
            strictSlash: true,
            routes: HashMap::new(),
        }
    }

    fn mapRoute(&mut self, route: Route) {
        self.routes.insert(route.httpMethod.clone().to_owned() + route.path, route);
    }

}

#[derive(Clone, Debug)]
pub struct Route {
    //Route path
    pub handler: fn(Request, Response),
    pub path: &'static str,
    //Route request type(GET, POST, UPDATE...)
    pub httpMethod: &'static str,
}


impl Route {

    pub fn new(path: &'static str, httpMethod: &'static str, handler: fn(Request, Response)) -> Route {
        Route {
            path: path,
            httpMethod: httpMethod,
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

    pub fn mapRoute(&mut self, route: Route) {
        self.router.mapRoute(route);
    }

    pub fn handleConnection(&self, mut _stream:  &TcpStream) -> std::io::Result<(Request, Response)> {
        let mut byteBuffer = [0; 2048];
        _stream.read(&mut byteBuffer).unwrap();
;
        let buffer = String::from_utf8_lossy(&byteBuffer[..]).to_string();
        let mut request = Request::new();

        let requestHeaderRegex = Regex::new(r"^(\w+) (\S+) HTTP/1.1").unwrap();
        let hostRegex = Regex::new(r"Host: (\S+)").unwrap();
        let contentTypeRegex = Regex::new(r"Content-Type: (\S+)").unwrap();
        let contentLengthRegex = Regex::new(r"content-length: (\d+)").unwrap();
        let contentRegex = Regex::new(r"Connection: (\S+)").unwrap();
        let userAgentRegex = Regex::new(r"User-Agent: (\S+)").unwrap();

        assert!(requestHeaderRegex.is_match(&buffer.to_string())); 

        let bufferwithStaticLifetime: &'static str = Box::leak(buffer.into_boxed_str());

        let mut request_method;
        match requestHeaderRegex.captures(bufferwithStaticLifetime) {
            Some(captures) => {
                request_method = captures.get(1).unwrap().as_str();
            },
            None => {
                request_method = "";
            }
        }

        let mut request_path;
        match requestHeaderRegex.captures(bufferwithStaticLifetime) {
            Some(captures) => {
                request_path = captures.get(2).unwrap().as_str();
            },
            None => {
                request_path = "";
            }
        }

        let mut host;
        match hostRegex.captures(bufferwithStaticLifetime) {
            Some(captures) => {
                host = captures.get(1).unwrap().as_str();
            },
            None => {
                host = "";
            }
        }

        let mut content_type;
        match contentTypeRegex.captures(bufferwithStaticLifetime) {
            Some(captures) => {
                content_type = captures.get(1).unwrap().as_str();
            },
            None => {
                content_type = "text/html";
            }
        }

        let mut content_length;
        match contentTypeRegex.captures(bufferwithStaticLifetime) {
            Some(captures) => {
                content_length = captures.get(1).unwrap().as_str();
            },
            None => {
                content_length = "0";
            }
        }

        let mut user_agent;
        match userAgentRegex.captures(bufferwithStaticLifetime) {
            Some(captures) => {
                user_agent = captures.get(1).unwrap().as_str();
            },
            None => {
                user_agent = "";
            }
        }

        let mut response = Response::new(_stream.try_clone()?);
        response.contentLength = content_length.parse::<usize>().unwrap();

        request.host = host;
        request.contentType = content_type;
        request.userAgent = user_agent;
        request.requestMethod = request_method;
        request.path = request_path;

        let mut content: &'static str;
        if let Some(content) = contentRegex.captures(bufferwithStaticLifetime) {
            request.body = &bufferwithStaticLifetime[content.get(1).unwrap().end() + 1..content.get(1).unwrap().end() + 1 + content_length.parse::<usize>().unwrap()];
        }

        Ok((request, response))
    }

    pub fn heartbeat(mut server: &mut Server) -> std::io::Result<()> {
        (*server).heartbeats += 1;
        println!("{}", (*server).heartbeats);
        Ok(())
    }

    pub fn start(&mut self) {
        println!("Running");
        println!("{:?}", self.get_listener().as_ref().unwrap());
        for stream in self.get_listener().as_ref().unwrap().incoming() {
            thread::scope(|s| {
                s.spawn(|_| {
                    let (req, mut res) = self.handleConnection(&stream.unwrap()).unwrap();
                    let result = self.router.routes.get(&(req.requestMethod.clone().to_owned() + req.path));
                    match result {
                        Some(route) => {
                            (route.handler)(req, res);
                        }, 
                        _ => {
                            res.sendStatus(StatusCode::NotFound);
                        }
                    }
                });
            });
        }
    }

}


