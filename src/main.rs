mod request;
mod response;
mod statuscode;
mod server;
mod serveroptions;
mod tests;
mod pool;
use server::Server;
use server::Router;
use server::Route;
use statuscode::StatusCode;
use std::fs;

extern crate regex;
extern crate crossbeam_utils;
extern crate scoped_threadpool;

fn main() {
    // run_test_server();
    run_simplest_server();
}

fn run_simplest_server() {
    let mut some_server: Server = Server::new(Router::new());
    let testGetRoute = Route::new("/test", "GET", |_req, mut res| {
        res.with_status(StatusCode::Ok).write_html(fs::read_to_string("test.html").expect("Something went wrong reading the file").as_bytes());
    });
    let testPostRoute = Route::new("/test", "POST", |_req, mut res| {
        println!("{}", _req.body);
        res.with_status(StatusCode::Ok).write_html(_req.body.as_bytes());
    });
    some_server.map_route(&testGetRoute);
    some_server.map_route(&testPostRoute);
    some_server.start();
}

fn run_test_server() {
    let mut some_server = Server::new(Router::new());
    let testGetRoute = Route::new("/test", "GET", |_req, mut res| {
        println!("{:?}", _req);
        let contents = fs::read_to_string("test.html").expect("Something went wrong reading the file");
        res.with_status(StatusCode::Accepted).write_html(contents.as_bytes());
        println!("{:?}", res);
    });
    some_server.map_route(&testGetRoute);
    some_server.heartbeat();
    println!("{}", some_server.heartbeats);
    some_server.start();
}