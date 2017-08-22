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

#[deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate ezo_ec;
extern crate ezo_ph;
extern crate ezo_rtd;
extern crate i2cdev;
extern crate neuras;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod config;
pub mod errors;
