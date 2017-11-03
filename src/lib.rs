//! # benita
//!
//! A collection of sensors and network sockets, deployed as services.
//!
//! The main goal is to generate a fast, memory safe, and concurrent environmental metrics
//! monitor.
//!
//! The _current_ included (aquatic) sensors are:
//!
//! * pH (made by Atlas Scientific: EZO PH)
//! * Electrical Conductivity (made by Atlas Scientific: EZO EC)
//! * Temperature (made by Atlas Scientific: EZO RTD)

#![recursion_limit = "1024"]

extern crate chrono;
extern crate clap;
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

#[macro_use]
// Useful items, functions and macros.
mod macros;

pub mod cli;
pub mod config;
pub mod devices;
pub mod errors;
pub mod network;
pub mod services;
pub mod utilities;
