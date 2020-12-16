use std::io::prelude::*;
use std::net::{ TcpStream };

use crate::statuscode::StatusCode;

#[derive(Debug)]
pub struct Response {
    pub content_length: usize,
    pub status_code: StatusCode,
    stream: TcpStream,
}

impl Response {
    
    pub fn new(mut _stream: TcpStream) -> Response {
        Response {
            content_length: 0,
            stream: _stream,
            status_code: StatusCode::Ok,
        }
    }

    pub fn with_status(&mut self, status_code: StatusCode) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn send_status(&mut self, status_code: StatusCode) {
        self.with_status(status_code);
        let response_header = format!(
                            "{}", "HTTP/1.1 ".to_owned() + &(self.status_code as u16).to_string() 
                            + " " + &self.status_code.to_string() + "\r\n\r\n"
        );
        self.write(response_header.as_bytes());
        self.complete();
    }


    pub fn write(&mut self, bytes: &[u8]) -> &mut Self {
        self.content_length += bytes.len();
        self.stream.write(bytes).unwrap();
        self
    }

    pub fn write_html(&mut self, bytes: &[u8]) -> &mut Self {
        let response_header = format!(
                            "{}", "HTTP/1.1 ".to_owned() + &(self.status_code as u16).to_string() 
                            + " " + &self.status_code.to_string() + "\r\n\r\n"
        );
        self.write(response_header.as_bytes());
        self.write(bytes);
        self.complete();
        self
    }

    pub fn complete(&mut self) {
        self.stream.flush().unwrap();
    }

}