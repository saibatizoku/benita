//! # benita
//!
//! Benita provides a network client to interact with sensors using `i2cdev`.
//! It is designed for usage on embedded devices with SoC (Systems On Chip)
//! with Linux, and personal computers in the network.
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

/// Command-line utilities and applications.
pub mod cli;
/// Configuration utilites and services.
pub mod config;
/// Error definitions.
pub mod errors;
/// Network utilities and services.
pub mod network;
/// Sensor utilities and services.
pub mod sensors;
