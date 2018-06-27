//! Networked services for Temperature sensing.
pub mod replies;
pub mod requests;

pub mod requester;
pub mod responder;

pub use self::requester::TemperatureRequester;
pub use self::responder::TemperatureResponder;
