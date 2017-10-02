//! Allows for remote command of the EC EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;

use benita::errors::*;
use benita::network::services::run_conductivity_server;

use clap::{App, Arg};

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-ec-rep")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about(
            "Benita IoT. A response service for electrical conductivity data.",
        )
        .arg(
            Arg::with_name("rep-url")
                .short("r")
                .long("rep-url")
                .value_name("REP_URL")
                .help("Sets the url for the response server")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .get_matches();

    let mut rep_url = String::new();

    if let Some(c) = matches.value_of("rep-url") {
        rep_url = String::from(c);
    }

    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);

    let _run = run_conductivity_server(&rep_url, &device_path, EZO_SENSOR_ADDR)?;

    // Never reach this line...
    Ok(())
}

fn main() {
    if let Err(ref e) = parse_cli_arguments() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}
