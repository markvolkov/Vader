mod statuscode;
mod server;
mod serveroptions;
mod tests;

use statuscode::StatusCode;
use server::Server;
use serveroptions::ServerOptions;

fn main() {

    StatusCode::printValues();
    let mut someServer = &mut Server::new();
    Server::heartbeat(someServer);
    
}
