//! Networked services for Conductivity sensing.
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

pub mod requests;
pub mod replies;

pub mod requester;
pub mod responder;

pub use self::requester::ConductivityRequester;
pub use self::responder::ConductivityResponder;
