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
extern crate crossbeam_utils;

fn main() {
    // run_test_server();
    run_simplest_server();
}

fn run_simplest_server() {
    let mut someServer = Server::new(Router::new());
    someServer.router.mapRoute(Route::new("/test", "GET", |_req, mut res| {
        res.withStatus(StatusCode::Ok).write_html(fs::read_to_string("test.html").expect("Something went wrong reading the file").as_bytes());
    }));
    someServer.start();
}

fn run_test_server() {
    let mut someServer = Server::new(Router::new());
    someServer.router.mapRoute(Route::new("/test", "GET", |_req, mut res| {
        println!("{:?}", _req);
        let contents = fs::read_to_string("test.html").expect("Something went wrong reading the file");
        res.withStatus(StatusCode::Accepted).write_html(contents.as_bytes());
        println!("{:?}", res);
    }));
    Server::heartbeat(&mut someServer);
    println!("{}", someServer.heartbeats);
    someServer.start()
}