#[cfg(test)]
mod tests {

    use std::net::{ Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream };
    use crate::server::Server;
    use crate::serveroptions::ServerOptions;

    #[test]
    pub fn testClient() -> () {
        // thread::spawn( move || server.handleConnection(stream?) );
        // let stream = TcpStream::connect("127.0.0.1:8080")
        //                .expect("Couldn't connect to the server...");
        // assert_eq!(stream.peer_addr().unwrap(), SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
    }

    #[test]
    pub fn testServer() -> () {
        let mut someServer = &mut Server::new();
        Server::heartbeat(someServer);
        assert_eq!(someServer.heartbeats, 1);
    }

    #[test]
    pub fn testClientAndServer() -> () {

    }

}