//! Allows for remote command of the EC EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;

use benita::cli::shared::is_url;
use benita::errors::*;
use benita::services::conductivity::ConductivitySensorService;

use clap::{App, Arg};

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-ec-rep")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. A response service for electrical conductivity data.")
        .arg(
            Arg::with_name("URL")
                .help("Sets the url for the response server")
                .takes_value(true)
                .validator(is_url)
                .required(true),
        )
        .arg(
            Arg::with_name("I2C")
                .help("Sets the path for the I2C sensor.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("ADDRESS")
                .help("Sets the I2C sensor address.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let mut rep_url = String::new();
    if let Some(c) = matches.value_of("URL") {
        rep_url = c.to_string();
    }

    let mut device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    if let Some(c) = matches.value_of("I2C") {
        device_path = c.to_string();
    }

    let mut address = EZO_SENSOR_ADDR;
    if let Some(c) = matches.value_of("ADDRESS") {
        address = c.parse().chain_err(|| "Bad Address")?;
    }

    // We initialize our service.
    let mut service = ConductivitySensorService::new(&rep_url, &device_path, address)
        .chain_err(|| "Could not create Conductivity service")?;

    {
        // This is the main loop, it will run for as long as the program runs.
        let _listen = service
            .listen()
            .chain_err(|| "Conductivity service stopped listening")?;
    }

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
