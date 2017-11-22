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

pub extern crate benita_ezo;

extern crate chrono;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate neuras;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod cli;
pub mod config;
pub mod errors;
pub mod utilities;

pub use benita_ezo as ezo;
