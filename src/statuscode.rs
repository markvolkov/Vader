#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum StatusCode {

    Continue = 100,
    SwitchingProtocols = 101,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthorized = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,

}

#[allow(dead_code)]
impl StatusCode {

    pub fn values() -> std::slice::Iter<'static, StatusCode> {
        static STATUSCODES: [StatusCode; 23] = [StatusCode::Continue, StatusCode::SwitchingProtocols, StatusCode::Ok, StatusCode::Created,StatusCode::Accepted, StatusCode::NonAuthorized, StatusCode::NoContent, 
        StatusCode::ResetContent, StatusCode::PartialContent, StatusCode::MultipleChoices, StatusCode::MovedPermanently, StatusCode::Forbidden, StatusCode::SeeOther,
        StatusCode::Found, StatusCode::NotModified, StatusCode::UseProxy, StatusCode::TemporaryRedirect, StatusCode::BadRequest, StatusCode::Unauthorized, StatusCode::PaymentRequired,
        StatusCode::NotFound, StatusCode::MethodNotAllowed, StatusCode::NotAcceptable];
        return STATUSCODES.iter();
    }

    pub fn print_values() -> () {
        for sc in StatusCode::values() {
            println!("{:?}", sc);
        }
    }

}

impl std::fmt::Display for StatusCode {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // fmt::Debug::fmt(self, f)
    }

}