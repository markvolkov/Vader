mod request;
mod response;
mod statuscode;
mod server;
mod serveroptions;
mod tests;
use server::Server;
use server::Router;
use server::Route;
use statuscode::StatusCode;
use std::fs;

extern crate regex;
extern crate crossbeam_utils;

fn main() {
    // run_test_server();
    run_simplest_server();
}

fn run_simplest_server() {
    let mut some_server = Server::new(Router::new());
    some_server.map_route(Route::new("/test", "GET", |_req, mut res| {
        res.with_status(StatusCode::Ok).write_html(fs::read_to_string("test.html").expect("Something went wrong reading the file").as_bytes());
    }));
    some_server.start();
}

fn run_test_server() {
    let mut some_server = Server::new(Router::new());
    some_server.map_route(Route::new("/test", "GET", |_req, mut res| {
        println!("{:?}", _req);
        let contents = fs::read_to_string("test.html").expect("Something went wrong reading the file");
        res.with_status(StatusCode::Accepted).write_html(contents.as_bytes());
        println!("{:?}", res);
    }));
    some_server.heartbeat();
    println!("{}", some_server.heartbeats);
    some_server.start()
}