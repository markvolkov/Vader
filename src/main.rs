mod request;
mod response;
mod statuscode;
mod server;
mod serveroptions;
mod tests;
use server::Server;
use server::Router;
use server::Route;
use serveroptions::ServerOptions;
use request::Request;
use response::Response;
use statuscode::StatusCode;
use std::fs;

extern crate regex;

fn main() {
    let mut someServer = Server::new(Router {
        strictSlash: false,
        testRoute: Route {
            //TODO Handle routes based on path and http method using a map handler
            path: "/test",
            httpMethod: "GET",
            handler: |req, mut res| {
                println!("{:?}", req);
                let contents = fs::read_to_string("test.html").expect("Something went wrong reading the file");
                res.withStatus(StatusCode::Accepted).write_html(contents.as_bytes());
                println!("{:?}", res);
            },
        }
    });
    Server::heartbeat(&mut someServer);
    println!("{}", someServer.heartbeats);
    Server::start(someServer);
}
