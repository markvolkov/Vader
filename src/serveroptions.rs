#[derive(Copy, Clone, Debug)]
pub struct ServerOptions {
    pub host: &'static str,
    pub port: usize,
}

impl ServerOptions {
    pub fn new() -> &'static Self {
        &ServerOptions {
            host: "127.0.0.1",
            port: 8080,
        }
    }
}