#[derive(Copy, Clone, Debug)]
pub struct Request {
    pub userAgent: &'static str,
    pub host: &'static str,
    pub path: &'static str,
    pub requestMethod: &'static str,
    pub contentType: &'static str,
    pub body: &'static str,
}

impl Request {

    pub fn new() -> Request {
        Request {
            host: "Default",
            path: "Default",
            requestMethod: "NONE",
            body: "Empty",
            userAgent: "Default",
            contentType: "text/html", 
        }
    }

}