use std::io::prelude::*;
use std::net::{ TcpStream };

use crate::statuscode::StatusCode;

#[derive(Debug)]
pub struct Response {
    pub contentLength: usize,
    pub statusCode: StatusCode,
    stream: TcpStream,
}

impl Response {
    
    pub fn new(mut _stream: TcpStream) -> Response {
        Response {
            contentLength: 0,
            stream: _stream,
            statusCode: StatusCode::Ok,
        }
    }

    pub fn withStatus(&mut self, statusCode: StatusCode) -> &mut Self {
        self.statusCode = statusCode;
        self
    }

    pub fn sendStatus(&mut self, statusCode: StatusCode) {
        self.withStatus(statusCode);
        let responseHeader = format!(
                            "{}", "HTTP/1.1 ".to_owned() + &(self.statusCode as u16).to_string() 
                            + " " + &self.statusCode.to_string() + "\r\n\r\n"
        );
        self.write(responseHeader.as_bytes());
        self.complete();
    }


    pub fn write(&mut self, bytes: &[u8]) -> &mut Self {
        self.contentLength += bytes.len();
        self.stream.write(bytes).unwrap();
        self
    }

    pub fn write_html(&mut self, bytes: &[u8]) -> &mut Self {
        let responseHeader = format!(
                            "{}", "HTTP/1.1 ".to_owned() + &(self.statusCode as u16).to_string() 
                            + " " + &self.statusCode.to_string() + "\r\n\r\n"
        );
        self.write(responseHeader.as_bytes());
        self.write(bytes);
        self.complete();
        self
    }

    pub fn complete(&mut self) {
        self.stream.flush().unwrap();
    }

}