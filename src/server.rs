use crate::serveroptions::ServerOptions;
use std::net::{ TcpListener, TcpStream };
use std::io::{ BufReader, BufRead, Write };
use std::thread;

pub struct Server {
    options: ServerOptions,
    listener: TcpListener,
}

pub struct Client {
    stream: TcpStream,
}

impl Server {

    pub fn new(opts: ServerOptions) -> Self {
        let portAsString: String = opts.port.to_string();
        let _listener = TcpListener::bind(opts.host.to_string() + portAsString).unwrap();
        Server {
            options: opts,
            listener: _listener,
        }
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

    pub fn start (&'static self) -> () {
        for stream in self.listener.incoming() {
            thread::spawn( move || self.handleConnection(stream?) );
        }
    }

}


