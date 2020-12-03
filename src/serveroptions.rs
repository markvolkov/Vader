pub struct ServerOptions {
    host: &'static str,
    port: usize,
}

impl ServerOptions {
    pub fn new() -> Self {
        ServerOptions {
            host: "127.0.0.1",
            port: 4000,
        }
    }
}