//! Allows for remote command of the EC EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;

use std::path::PathBuf;

use benita::cli::shared::is_url;
use benita::config::{SensorConfig, SocketConfig};
use benita::errors::*;
use benita::services::conductivity::ConductivitySensorService;

use clap::{App, Arg};

// Main code. Parse the command-line arguments and execute.
fn run_main_code() -> Result<()> {
    // Match the command-line arguments from std::io and start the service.
    let matches = App::new("conductivity-command-server")
        .version("0.2.0")
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

    // Blank socket configuration.
    let mut socket_cfg = SocketConfig::default();

    // next, add it the `url` from the command-line
    if let Some(c) = matches.value_of("URL") {
        socket_cfg.url = c;
    }

    // Blank socket configuration.
    let mut sensor_cfg = SensorConfig::default();

    // next, add it the `I2CDEV` path from the command-line
    if let Some(c) = matches.value_of("I2C") {
        sensor_cfg.path = PathBuf::from(c);
    }

    // next, add it the `I2C ADDRESS` from the command-line
    if let Some(c) = matches.value_of("ADDRESS") {
        sensor_cfg.address = c.parse().chain_err(|| "Bad Address")?;
    }

    // We initialize the conductivity sensor service.
    let mut service = ConductivitySensorService::new(socket_cfg, sensor_cfg)
        .chain_err(|| "Could not create Conductivity service")?;

    // This is the main loop, it will run for as long as the program runs.
    {
        // Start listening on the specified `URL` for incoming requests.
        let _listen = service
            .listen()
            .chain_err(|| "Conductivity service stopped listening")?;
    }

    // Never reach this line...
    Ok(())
}

fn main() {
    // Standard setup to catch any errors.
    if let Err(ref e) = run_main_code() {
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
