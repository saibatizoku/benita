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
extern crate chrono;
extern crate clap;
#[macro_use]
extern crate failure;
extern crate neuras;
extern crate url;
extern crate zmq;

pub extern crate benita_ezo;

pub mod cli;
pub mod errors;
pub mod utilities;

pub use benita_ezo as ezo;
