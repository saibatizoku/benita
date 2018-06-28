//! Submodule that provides functionality for EZO water sensor chips.
extern crate ezo_common;
extern crate ezo_ec;
extern crate ezo_ph;
extern crate ezo_rtd;
#[macro_use]
extern crate failure;
extern crate i2cdev;
#[macro_use]
extern crate log;
extern crate neuras;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate zmq;

#[macro_use]
pub mod devices;
#[macro_use]
pub mod network;

pub mod config;
pub mod errors;
pub mod utilities;

pub mod common_ezo;

pub mod conductivity;
pub mod ph;
pub mod temperature;
