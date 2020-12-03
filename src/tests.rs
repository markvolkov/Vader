#[cfg(test)]
mod tests {

    use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream };
    use crate::StatusCode;
    use crate::Server;
    use crate::ServerOptions;
    use super::*;

    #[test]
    pub fn testClient() -> () {
        let dummyServer = Server::new(ServerOptions::new()).start();
        let stream = TcpStream::connect("127.0.0.1:4000")
                       .expect("Couldn't connect to the server...");
        assert_eq!(stream.peer_addr().unwrap(), SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4000)));
    }

    #[test]
    pub fn testServer() -> () {

    }

    #[test]
    pub fn testClientAndServer() -> () {

    }

}