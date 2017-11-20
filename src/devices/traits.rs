//! Device traits

/// A request sent over a socket
pub trait SensorCommand
where
    Self: ::std::marker::Sized,
{
    type Error;
    /// The expected response type.
    type Response: SensorResponse;

    /// Create a new instance from `&str`.
    fn from_request_str(req_str: &str) -> ::std::result::Result<Self, Self::Error>;
    /// Return the instance as a `String`.
    fn to_request_string(&self) -> String;
    // Execute the request over the socket, and return the corresponding response.
    //fn send_to<T: Endpoint>(&self, &T) -> ::std::result::Result<Self::Response, , Self::Error>;
}

/// A response sent over a socket
pub trait SensorResponse
where
    Self: ::std::marker::Sized,
{
    type Error;

    /// Create a new instance from `&str`.
    fn parse_response(&str) -> ::std::result::Result<Self, Self::Error>;
    /// Return the instance as a `String`.
    fn to_reply_string(&self) -> String;
    // Receive and parse the reply from the network.
    //fn recv_from<T: Endpoint>(&T) -> ::std::result::Result<Self, Self::Error>;
}
