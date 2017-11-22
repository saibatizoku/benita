//! Allows for remote command of the RTD EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate fern;
#[macro_use]
extern crate log;
extern crate neuras;

use std::path::PathBuf;

use benita::cli::shared::is_url;
use benita::ezo::common_ezo::EzoChipAPI;
use benita::ezo::config::{ConnectionType, SensorConfig, SocketConfig};
use benita::ezo::errors::*;
use benita::ezo::network::{Endpoint, ReplyStatus};
use benita::ezo::temperature::TemperatureAPI;
use benita::ezo::temperature::device::TemperatureSensor;
use benita::ezo::temperature::network::TemperatureResponder;
use benita::ezo::temperature::command::*;
use benita::ezo::utilities::*;

use clap::{App, Arg};

// Configure and start logger.
fn start_logger() -> Result<()> {
    let _logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("temperature-responder.log")
            .chain_err(|| "failed to open log file")?)
        .apply()
        .chain_err(|| "Could not setup logging")?;
    Ok(())
}

// Return a `Socket` from a `SocketConfig`
fn socket_from_config(cfg: &SocketConfig) -> Result<neuras::zmq::Socket> {
    let socket = match cfg.socket_connection {
        ConnectionType::Bind => create_and_bind_responder(cfg.url)?,
        ConnectionType::Connect => create_and_connect_responder(cfg.url)?,
    };
    Ok(socket)
}

// Return 'err' string, and log it
fn return_error(e: Error) -> String {
    error!("temperature sensor error: {}", e);
    format!("{:?}", ReplyStatus::Err)
}

// Match and evaluate commands
fn match_and_eval(s: &str, e: &TemperatureResponder) -> Result<String> {
    match s {
        a if <CalibrationState as I2CCommand>::from_str(a).is_ok() => {
            let _req = <CalibrationState as I2CCommand>::from_str(s)?;
            let reply = match e.get_calibration_status() {
                Ok(rep) => format!("{:?}", rep),
                Err(e) => return_error(e),
            };
            Ok(reply)
        }
        a if <DeviceInformation as I2CCommand>::from_str(a).is_ok() => {
            let _req = <DeviceInformation as I2CCommand>::from_str(s)?;
            let reply = match e.get_device_info() {
                Ok(rep) => format!("{:?}", rep),
                Err(e) => return_error(e),
            };
            Ok(reply)
        }
        a if <Reading as I2CCommand>::from_str(a).is_ok() => {
            let _req = <Reading as I2CCommand>::from_str(s)?;
            let reply = match e.get_reading() {
                Ok(rep) => format!("{:?}", rep),
                Err(e) => return_error(e),
            };
            Ok(reply)
        }
        a if <Sleep as I2CCommand>::from_str(a).is_ok() => {
            let _req = <Sleep as I2CCommand>::from_str(s)?;
            let reply = match e.set_sleep() {
                Ok(rep) => format!("{:?}", rep),
                Err(e) => return_error(e),
            };
            Ok(reply)
        }
        a if <Status as I2CCommand>::from_str(a).is_ok() => {
            let _req = <Status as I2CCommand>::from_str(s)?;
            let reply = match e.get_device_status() {
                Ok(rep) => format!("{:?}", rep),
                Err(e) => return_error(e),
            };
            Ok(reply)
        }
        _ => {
            error!("bad sensor command");
            Ok(format!("{:?}", ReplyStatus::Err))
        }
    }
}

// Parse the command-line arguments and execute.
fn evaluate_command_line() -> Result<()> {
    // Read args from stdin and match to our application.
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

    // socket configuration from args.
    let socket_cfg = SocketConfig {
        socket_connection: ConnectionType::Bind,
        url: matches.value_of("URL").unwrap(),
    };

    // sensor configuration from args.
    let sensor_cfg = SensorConfig {
        address: matches
            .value_of("ADDRESS")
            .unwrap()
            .parse()
            .chain_err(|| "Bad Address")?,
        path: PathBuf::from(matches.value_of("I2C").unwrap()),
    };

    // initialize the sensor.
    let sensor = TemperatureSensor::from_config(sensor_cfg)?;

    // initialize the socket.
    let socket = socket_from_config(&socket_cfg)?;

    // initialize the responder with the sensor and socket.
    let responder = TemperatureResponder::new(socket, sensor)?;

    // the main loop, it will run for as long as the program runs.
    loop {
        let req_str = &responder.recv()?;
        info!("REQ: {}", &req_str);
        let call: String = match_and_eval(&req_str, &responder)?;
        info!("REP: {}", &call);
        let _reply = &responder.send(call.as_bytes())?;
    }

    // Never reach this line...
    // Ok(())
}

// Main program. Starts logger, then evaluates args from stdin.
fn run_code() -> Result<()> {
    // Initialize logging.
    let _log = start_logger()?;
    info!("Starting calibrated-service");
    evaluate_command_line()
}

// fn main() wrapped to handle error chains
quick_main!(run_code);
