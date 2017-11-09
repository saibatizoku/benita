//! Networked services for Temperature sensing.
pub mod replies;
pub mod requests;

mod requester;
mod responder;

pub use self::requester::TemperatureRequester;
pub use self::responder::TemperatureResponder;
