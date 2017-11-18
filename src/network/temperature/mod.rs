//! Networked services for Temperature sensing.
pub mod errors {
    use super::replies;
    use super::requests;
    use super::requester;
    use super::responder;

    error_chain! {
        links {
            Replies(replies::errors::Error, replies::errors::ErrorKind);
            Requests(requests::errors::Error, requests::errors::ErrorKind);
            Requester(requester::errors::Error, requester::errors::ErrorKind);
            Responder(responder::errors::Error, responder::errors::ErrorKind);
        }
    }
}

pub mod replies;
pub mod requests;

pub mod requester;
pub mod responder;

pub use self::requester::TemperatureRequester;
pub use self::responder::TemperatureResponder;
