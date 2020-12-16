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

    pub fn mapRoute(&mut self, route: Route) {
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

    pub fn handleConnection(&self, mut _stream:  &TcpStream) -> std::io::Result<(Request, Response)> {
        let mut byteBuffer = [0; 2048];
        _stream.read(&mut byteBuffer).unwrap();
        // println!("Request: \n{}", String::from_utf8_lossy(&byteBuffer[..]));
        let buffer = String::from_utf8_lossy(&byteBuffer[..]).to_string();
        let mut request = Request::new();
        //Assuming we are only getting valid input
        let requestHeaderRegex = Regex::new(r"^(\w+) (\S+) HTTP/1.1").unwrap();
        let hostRegex = Regex::new(r"Host: (\S+)").unwrap();
        let contentTypeRegex = Regex::new(r"Content-Type: (\S+)").unwrap();
        let userAgentRegex = Regex::new(r"User-Agent: (\S+)").unwrap();
        let contentLengthRegex = Regex::new(r"content-length: (\d+)").unwrap();
        let contentRegex = Regex::new(r"Connection: (\S+)").unwrap();

        assert!(requestHeaderRegex.is_match(&buffer.to_string())); 

        let bufferwithStaticLifetime: &'static str = Box::leak(buffer.into_boxed_str());
        let headerCaptures = requestHeaderRegex.captures(bufferwithStaticLifetime).ok_or("Error parsing request...").unwrap();
        let hostCaptures = hostRegex.captures(bufferwithStaticLifetime).ok_or("Error parsing request...").unwrap();
        let contentTypeCaptures = contentTypeRegex.captures(bufferwithStaticLifetime).ok_or("Error parsing request...").unwrap();
        let contentLengthCaptures = contentLengthRegex.captures(bufferwithStaticLifetime).ok_or("Error parsing request...").unwrap();
        let contentCaptures = contentRegex.captures(bufferwithStaticLifetime).ok_or("Error parsing request...").unwrap();
        let userAgentCaptures = userAgentRegex.captures(bufferwithStaticLifetime).ok_or("Error parsing request...").unwrap();

        let host = hostCaptures.get(1).unwrap().as_str();
        let contentType = contentTypeCaptures.get(1).unwrap().as_str();
        let contentLength = contentLengthCaptures.get(1).unwrap().as_str();
        let userAgent = userAgentCaptures.get(1).unwrap().as_str();

        let mut response = Response::new(_stream.try_clone()?);
        response.contentLength = contentLength.parse::<usize>().unwrap();

        request.host = host;
        request.contentType = contentType;
        request.userAgent = userAgent;
        request.requestMethod = headerCaptures.get(1).unwrap().as_str();
        request.path = headerCaptures.get(2).unwrap().as_str(); 
        request.body = &bufferwithStaticLifetime[contentCaptures.get(1).unwrap().end() + 1
                       .. contentCaptures.get(1).unwrap().end() + 1 + contentLength.parse::<usize>().unwrap()];
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


