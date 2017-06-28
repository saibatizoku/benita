//! # benita
//!
//! Benita provides a client to interact with sensors over I2C. It is designed
//! for usage on embedded devices with SoC (Systems On Chip) with Linux.
//!
//! The included aquatic sensors are:
//!
//! * pH
//! * Electrical Conductivity
//! * Temperature
extern crate i2cdev;

mod sensors;

use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

#[derive(Clone)]
pub struct SlaveDevice {
    bus: u8,
    address: u16,
}

pub trait I2cCommand {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

pub trait I2cSlave {
    fn new(bus: u8, address: u16) -> Self;
    fn send<T: I2cCommand>(&self, cmd: T) -> Result<(), LinuxI2CError>;
}

impl I2cSlave for SlaveDevice {
    fn new(bus: u8, address: u16) -> SlaveDevice {
        SlaveDevice { bus, address }
    }
    fn send<T: I2cCommand>(&self, cmd: T) -> Result<(), LinuxI2CError> {
        let bus = format!("/dev/i2c-{}", self.bus);
        let mut dev = try!(LinuxI2CDevice::new(bus, self.address));
        dev.write(&cmd.to_bytes())
    }
}
