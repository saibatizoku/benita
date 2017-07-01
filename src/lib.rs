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
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate i2cdev;

pub mod errors;
mod sensors;

use errors::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

#[derive(Clone)]
pub struct SensingDevice {
    bus: u8,
    address: u16,
}

pub trait I2cCommand {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

pub trait I2cSensing {
    fn new(bus: u8, address: u16) -> Self;
    fn send<T: I2cCommand>(&self, cmd: T) -> Result<()>;
}

impl I2cSensing for SensingDevice {
    fn new(bus: u8, address: u16) -> SensingDevice {
        SensingDevice { bus, address }
    }
    fn send<T: I2cCommand>(&self, cmd: T) -> Result<()> {
        let bus = format!("/dev/i2c-{}", self.bus);
        let mut dev = LinuxI2CDevice::new(bus, self.address)
                       .chain_err(|| "Could not open I2C device")?;
        dev.write(&cmd.to_bytes()).chain_err(|| "Could not send command")
    }
}
