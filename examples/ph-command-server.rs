//! Allows for remote command of the PH EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.
extern crate benita;
extern crate chrono;
extern crate clap;
extern crate failure;
extern crate fern;
#[macro_use]
extern crate log;
extern crate neuras;
extern crate zmq;

use std::path::PathBuf;
use std::result;

use benita::cli::is_url;
use benita::ezo::common_ezo::EzoChipAPI;
use benita::ezo::config::{ConnectionType, SensorConfig, SocketConfig};
use benita::ezo::errors::{Error as EzoError};
use benita::ezo::network::{Endpoint, ReplyStatus};
use benita::ezo::ph::command::*;
use benita::ezo::ph::device::PhSensor;
use benita::ezo::ph::network::PhResponder;
use benita::ezo::ph::PhAPI;
use benita::ezo::utilities::*;

use clap::{App, Arg};
use failure::{Error, ResultExt};
use zmq::Socket;

type Result<T> = result::Result<T, Error>;

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
        .chain(fern::log_file("ph-responder.log").context("failed to open log file")?)
        .apply()
        .context("Could not setup logging")?;
    Ok(())
}

// Return a `Socket` from a `SocketConfig`
fn socket_from_config(cfg: &SocketConfig) -> Result<Socket> {
    let socket = match cfg.socket_connection {
        ConnectionType::Bind => create_and_bind_responder(cfg.url)?,
        ConnectionType::Connect => create_and_connect_responder(cfg.url)?,
    };
    Ok(socket)
}

// Return 'err' string, and log it
fn return_error(e: EzoError) -> String {
    error!("ph sensor error: {}", e);
    format!("{:?}", ReplyStatus::Err)
}

// Match and evaluate commands
fn match_and_eval(s: &str, e: &PhResponder) -> Result<String> {
    match s {
        a if <CalibrationState as I2CCommand>::from_str(a).is_ok() => {
            let _req = <CalibrationState as I2CCommand>::from_str(s)?;
            let reply = match e.get_calibration_status() {
                Ok(rep) => format!("{:?}", rep),
                Err(e) => return_error(e.into()),
            };
            Ok(reply)
        }
        a if <CompensationSet as I2CCommand>::from_str(a).is_ok() => {
            let _req = <CompensationSet as I2CCommand>::from_str(s)?;
            let reply = match e.set_compensation(_req.0) {
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
    let matches = App::new("benita-ph-network-service")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. A network service for pH data.")
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
            .parse::<u16>()
            .context("Bad Address")?,
        path: PathBuf::from(matches.value_of("I2C").unwrap()),
    };

    // initialize the sensor.
    let sensor = PhSensor::from_config(sensor_cfg)?;

    // initialize the socket.
    let socket = socket_from_config(&socket_cfg)?;

    // initialize the responder with the sensor and socket.
    let responder = PhResponder::new(socket, sensor)?;

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

fn main() {
    if let Err(ref e) = run_code() {
        println!("error: {:?}", e.cause());
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        let backtrace = e.backtrace();
        println!("backtrace: {:?}", backtrace);
        ::std::process::exit(1);
    }
}
