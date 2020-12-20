#[derive(Copy, Clone, Debug)]
pub struct ServerOptions<'a> {
    pub host: &'a str,
    pub port: usize,
}

impl<'a> ServerOptions<'a> {
    pub fn new<'b>() -> &'a Self {
        &ServerOptions {
            host: "127.0.0.1",
            port: 8080,
        }
    }
}