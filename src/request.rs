#[derive(Copy, Clone, Debug)]
pub struct Request {
    pub user_agent: &'static str,
    pub host: &'static str,
    pub path: &'static str,
    pub request_method: &'static str,
    pub content_type: &'static str,
    pub body: &'static str,
}

impl Request {

    pub fn new() -> Request {
        Request {
            host: "Default",
            path: "Default",
            request_method: "NONE",
            body: "",
            user_agent: "Default",
            content_type: "text/html", 
        }
    }

}