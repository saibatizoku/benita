//! Common network items.
use std;

use errors::*;

/// A response sent over a socket
pub trait Endpoint
where
    Self: std::marker::Sized,
{
    /// bind the endpoint to the given `url`. Listens for incoming messages.
    fn bind(&self, url: &str) -> Result<()>;
    /// connect the endpoint to the given `url`. Sends outgoing messages.
    fn connect(&self, url: &str) -> Result<()>;
    /// Send a slice of bytes to the endpoint.
    fn send(&self, msg: &[u8]) -> Result<()>;
    /// Receive a `String` from the endpoint.
    fn recv(&self) -> Result<String>;
}

/// A request sent over a socket
pub trait SocketRequest
where
    Self: std::marker::Sized,
{
    /// The expected response type.
    type Response: SocketReply;

    /// Create a new instance from `&str`.
    fn from_request_str(req_str: &str) -> Result<Self>;
    /// Return the instance as a `String`.
    fn to_request_string(&self) -> String;
    /// Execute the request over the socket, and return the corresponding response.
    fn send_to<T: Endpoint>(&self, &T) -> Result<Self::Response>;
}

/// A response sent over a socket
pub trait SocketReply
where
    Self: std::marker::Sized,
{
    /// Create a new instance from `&str`.
    fn parse_response(&str) -> Result<Self>;
    /// Return the instance as a `String`.
    fn to_reply_string(&self) -> String;
    /// Receive and parse the reply from the network.
    fn recv_from<T: Endpoint>(&T) -> Result<Self>;
}

/// `ok` reply.
#[derive(PartialEq)]
pub enum ReplyStatus {
    Ok,
    Err,
}

impl ReplyStatus {
    fn parse(rep_str: &str) -> Result<ReplyStatus> {
        match rep_str {
            "ok" => Ok(ReplyStatus::Ok),
            "err" => Ok(ReplyStatus::Err),
            _ => Err(ErrorKind::ResponseParse.into()),
        }
    }
}

impl std::fmt::Debug for ReplyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReplyStatus::Ok => write!(f, "ok"),
            ReplyStatus::Err => write!(f, "err"),
        }
    }
}

impl std::fmt::Display for ReplyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_SocketReply_for!(ReplyStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_status_reply_from_valid_str() {
        let reply = ReplyStatus::parse_response("ok").unwrap();
        assert_eq!(reply, ReplyStatus::Ok);

        let reply = ReplyStatus::parse_response("err").unwrap();
        assert_eq!(reply, ReplyStatus::Err);
    }

    #[test]
    fn create_status_reply_from_invalid_str_yields_err() {
        let reply = ReplyStatus::parse_response("okerr");
        assert!(reply.is_err());
    }
}
