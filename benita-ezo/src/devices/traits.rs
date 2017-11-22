//! Device traits
use errors::*;

use i2cdev::linux::LinuxI2CDevice;

/// A marker for sensor devices
pub trait SensorDevice<T>
where
    Self: ::std::marker::Sized,
{
    type Error;

    /// Mutable I2C device.
    fn i2c_mut(&self) -> ::std::cell::RefMut<LinuxI2CDevice>;

    /// Read a given `I2CResponse` from the device.
    fn read<R: I2CResponse>(&self, response: &R) -> ::std::result::Result<(), Error> {
        unimplemented!();
    }

    /// Write a given `I2CCommand` to the device.
    fn write<C: I2CCommand>(&self, cmd: &C) -> ::std::result::Result<(), Error> {
        unimplemented!();
    }
    /// Write a given `I2CCommand` to the device.
    fn run<C: I2CCommand>(&self, cmd: &C) -> ::std::result::Result<(), Error> {
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
    fn write<A, T: SensorDevice<A>>(&self, &T) -> ::std::result::Result<Self::Response, Error>;
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
}
