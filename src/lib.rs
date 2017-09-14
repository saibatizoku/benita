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

extern crate clap;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate ezo_common;
extern crate ezo_ec;
extern crate ezo_ph;
extern crate ezo_rtd;
extern crate i2cdev;
extern crate neuras;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod cli;
pub mod config;
pub mod errors;
pub mod network;
pub mod sensors;
