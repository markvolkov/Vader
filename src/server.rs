use crate::serveroptions::ServerOptions;
use std::net::{ TcpListener, TcpStream };
use std::io::{ BufReader, BufRead, Write };
use std::thread;

#[derive(Copy, Clone, Debug)]
pub struct Server {
    pub options: &'static ServerOptions,
    pub heartbeats: usize,
}

pub struct Client {
    pub stream: TcpStream,
}

impl Server {

    pub fn new() -> Server {
        Server {
            options: ServerOptions::new(),
            heartbeats: 0,
        }
    }

    pub fn get_listener(&self) -> Option<TcpListener> {
        let connection: &'static str = "127.0.0.1:8080";
        Some(TcpListener::bind(connection).unwrap())
    }
    
    pub fn serve(&self, _stream: TcpStream) -> () {
        let mut stream = BufReader::new(_stream);
        loop {
            //not str - str is an immutable datastructure on the stack, and String is mutable and heap allocated
            let mut buffer = String::new();
            if stream.read_line(&mut buffer).is_err() {
                // return Err(std::io::Error::new("Error while serving client."))
                println!("Error while serving client."); //TODO: Make better error handling
                break;
            } else {
                stream.get_ref().write(buffer.as_bytes())
                      .unwrap();
            }
        }
    }

    pub fn handleConnection(&self, _stream: TcpStream) -> std::io::Result<()> {
        self.serve(_stream);
        Ok(())
    }

    pub fn heartbeat(mut server: &mut Server) -> std::io::Result<()> {
        (*server).heartbeats += 1;
        println!("{}", (*server).heartbeats);
        Ok(())
    }

    pub fn start (mut server: Server) -> () {
        thread::spawn(move || 
            for stream in server.get_listener().as_ref().unwrap().incoming() {
                thread::spawn( move || server.handleConnection(stream?) );
            }
        );
    }

}


