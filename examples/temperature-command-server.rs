//! Allows for remote command of the RTD EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;
#[macro_use]
extern crate error_chain;

use std::path::PathBuf;

use benita::cli::shared::is_url;
use benita::config::{ConnectionType, SensorConfig, SocketConfig};
use benita::devices::temperature::TemperatureSensor;
use benita::errors::*;
use benita::network::common::{Endpoint, SocketRequest};
use benita::network::temperature::TemperatureResponder;
use benita::network::temperature::requests::*;
use benita::utilities::*;

use clap::{App, Arg};

// Match and evaluate commands
fn match_and_eval(s: &str, e: &mut TemperatureResponder) -> Result<String> {
    println!("REQ: {}", &s);
    match s {
        a if CalibrationState::from_request_str(a).is_ok() => {
            let _req = CalibrationState::from_request_str(s)?;
            let reply = e.get_calibration_status()?;
            Ok(format!("{:?}", reply))
        }
        a if DeviceInformation::from_request_str(a).is_ok() => {
            let _req = DeviceInformation::from_request_str(s)?;
            let reply = e.get_device_info()?;
            Ok(format!("{:?}", reply))
        }
        a if Reading::from_request_str(a).is_ok() => {
            let _req = Reading::from_request_str(s)?;
            let reply = e.get_reading()?;
            Ok(format!("{:?}", reply))
        }
        a if Sleep::from_request_str(a).is_ok() => {
            let _req = Sleep::from_request_str(s)?;
            let reply = e.set_sleep()?;
            Ok(format!("{:?}", reply))
        }
        a if Status::from_request_str(a).is_ok() => {
            let _req = Status::from_request_str(s)?;
            let reply = e.get_device_status()?;
            Ok(format!("{:?}", reply))
        }
        _ => bail!("unrecognized command"),
    }
}

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

    let mut socket_cfg = SocketConfig {
        socket_connection: ConnectionType::Bind,
        ..Default::default()
    };

    if let Some(c) = matches.value_of("URL") {
        socket_cfg.url = c;
    }

    let mut sensor_cfg = SensorConfig::default();

    if let Some(c) = matches.value_of("I2C") {
        sensor_cfg.path = PathBuf::from(c);
    }

    if let Some(c) = matches.value_of("ADDRESS") {
        sensor_cfg.address = c.parse().chain_err(|| "Bad Address")?;
    }

    // We initialize the sensor.
    let sensor = TemperatureSensor::from_config(sensor_cfg)?;

    // We initialize the socket.
    let socket = match socket_cfg.socket_connection {
        ConnectionType::Bind => create_and_bind_responder(socket_cfg.url)?,
        ConnectionType::Connect => create_and_connect_responder(socket_cfg.url)?,
    };

    // We initialize the responder with the sensor and socket.
    let mut responder = TemperatureResponder::new(socket, sensor)?;

    // This is the main loop, it will run for as long as the program runs.
    loop {
        let req_str = &responder.recv()?;
        println!("REQ: {}", &req_str);
        let call: String = match_and_eval(&req_str, &mut responder)?;
        println!("REP: {}", &call);
        let _reply = &responder.send(call.as_bytes())?;
    }

    // Never reach this line...
    // Ok(())
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
