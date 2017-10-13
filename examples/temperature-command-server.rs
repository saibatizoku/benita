//! Allows for remote command of the RTD EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;

use benita::cli::shared::is_url;
use benita::config::{SensorConfig, SocketConfig};
use benita::errors::*;
use benita::services::temperature::TemperatureSensorService;

use clap::{App, Arg};

// Parse the command-line arguments and execute.
fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-temperature-network-service")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. A network service for temperature data.")
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

    let mut socket_cfg = SocketConfig::default();

    if let Some(c) = matches.value_of("URL") {
        socket_cfg.url = c;
    }

    let mut sensor_cfg = SensorConfig::default();

    if let Some(c) = matches.value_of("I2C") {
        sensor_cfg.path = c;
    }

    if let Some(c) = matches.value_of("ADDRESS") {
        sensor_cfg.address = c.parse().chain_err(|| "Bad Address")?;
    }

    // We initialize our service.
    let mut service = TemperatureSensorService::new(socket_cfg, sensor_cfg)
        .chain_err(|| "Could not create Temperature service")?;

    {
        // This is the main loop, it will run for as long as the program runs.
        let _listen = service
            .listen()
            .chain_err(|| "Temperature service stopped listening")?;
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
