//! Device traits
use errors::*;

/// A marker for sensor devices
pub trait SensorDevice
where
    Self: ::std::marker::Sized,
{
    type Error;
    /// Read a given `I2CResponse` from the device.
    fn read<R: I2CResponse>(&self, response: &R) -> ::std::result::Result<(), Error> {
        unimplemented!();
    }

    /// Write a given `I2CCommand` to the device.
    fn write<T: I2CCommand>(&self, cmd: &T) -> ::std::result::Result<(), Error> {
        unimplemented!();
    }
}

/// A request sent over a socket
pub trait I2CCommand
where
    Self: ::std::marker::Sized,
{
    /// The expected response type.
    type Response: I2CResponse;

    /// Create a new instance from `&str`.
    fn from_str(req_str: &str) -> ::std::result::Result<Self, Error>;
    /// Return the instance as a `String`.
    fn to_string(&self) -> String;
    /// Execute the request over the socket, and return the corresponding response.
    fn write<T: SensorDevice>(&self, &T) -> ::std::result::Result<Self::Response, Error>;
}

/// A response sent over a socket
pub trait I2CResponse
where
    Self: ::std::marker::Sized,
{
    /// Create a new instance from `&str`.
    fn from_str(&str) -> ::std::result::Result<Self, Error>;
    /// Return the instance as a `String`.
    fn to_string(&self) -> String;
    /// Receive and parse the reply from the network.
    fn read<T: SensorDevice>(&T) -> ::std::result::Result<Self, Error>;
}
