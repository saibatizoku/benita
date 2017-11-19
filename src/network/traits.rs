//! Network traits
use errors::*;

/// A response sent over a socket
pub trait Endpoint
where
    Self: ::std::marker::Sized,
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
    Self: ::std::marker::Sized,
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
    Self: ::std::marker::Sized,
{
    /// Create a new instance from `&str`.
    fn parse_response(&str) -> Result<Self>;
    /// Return the instance as a `String`.
    fn to_reply_string(&self) -> String;
    /// Receive and parse the reply from the network.
    fn recv_from<T: Endpoint>(&T) -> Result<Self>;
}
