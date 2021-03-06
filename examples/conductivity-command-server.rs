//! Allows for remote command of the EC EZO chip, exposing a limited API.
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

use std::fmt;
use std::path::PathBuf;
use std::result;

use benita::cli::is_url;
use benita::ezo::conductivity::device::commands::Command;
use benita::ezo::conductivity::device::ConductivitySensor;
use benita::ezo::conductivity::network::requests::*;
use benita::ezo::conductivity::network::ConductivityResponder;
use benita::ezo::config::{ConnectionType, SensorConfig, SocketConfig};
use benita::ezo::network::{Endpoint, ReplyStatus, SocketRequest};
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
        .chain(
            fern::log_file("conductivity-responder.log").context("failed to open log file")?,
        )
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

// Evaluate a conductivity command using the given responder. Returns a String.
fn eval<T: Command>(responder: &ConductivityResponder, cmd: T) -> Result<String>
where
    <T as Command>::Response: fmt::Debug,
{
    let reply = match cmd.run(&mut responder.sensor.i2cdev.borrow_mut()) {
        Ok(rep) => format!("{:?}", rep),
        Err(_) => format!("{:?}", ReplyStatus::Err),
    };
    Ok(reply)
}

// Match and evaluate commands.
//
// NOTE: It uses a custom subset of the `ConducivityAPI`.
fn match_and_eval(s: &str, e: &ConductivityResponder) -> Result<String> {
    match s {
        a if CalibrationState::from_str(a).is_ok() => eval(e, CalibrationState::from_str(a)?),
        a if CompensationGet::from_str(a).is_ok() => eval(e, CompensationGet::from_str(a)?),
        a if CompensationSet::from_str(a).is_ok() => eval(e, CompensationSet::from_str(a)?),
        a if DeviceInformation::from_str(a).is_ok() => eval(e, DeviceInformation::from_str(a)?),
        a if LedOff::from_str(a).is_ok() => eval(e, LedOff::from_str(a)?),
        a if LedOn::from_str(a).is_ok() => eval(e, LedOn::from_str(a)?),
        a if LedState::from_str(a).is_ok() => eval(e, LedState::from_str(a)?),
        a if Export::from_str(a).is_ok() => eval(e, Export::from_str(a)?),
        a if ExportInfo::from_str(a).is_ok() => eval(e, ExportInfo::from_str(a)?),
        a if Import::from_str(a).is_ok() => eval(e, Import::from_str(a)?),
        a if OutputEnableConductivity::from_str(a).is_ok() => {
            eval(e, OutputEnableConductivity::from_str(a)?)
        }
        a if OutputEnableTds::from_str(a).is_ok() => eval(e, OutputEnableTds::from_str(a)?),
        a if OutputEnableSalinity::from_str(a).is_ok() => {
            eval(e, OutputEnableSalinity::from_str(a)?)
        }
        a if OutputEnableSpecificGravity::from_str(a).is_ok() => {
            eval(e, OutputEnableSpecificGravity::from_str(a)?)
        }
        a if OutputDisableConductivity::from_str(a).is_ok() => {
            eval(e, OutputDisableConductivity::from_str(a)?)
        }
        a if OutputDisableTds::from_str(a).is_ok() => eval(e, OutputDisableTds::from_str(a)?),
        a if OutputDisableSalinity::from_str(a).is_ok() => {
            eval(e, OutputDisableSalinity::from_str(a)?)
        }
        a if OutputDisableSpecificGravity::from_str(a).is_ok() => {
            eval(e, OutputDisableSpecificGravity::from_str(a)?)
        }
        a if OutputState::from_str(a).is_ok() => eval(e, OutputState::from_str(a)?),
        a if ProtocolLockDisable::from_str(a).is_ok() => eval(e, ProtocolLockDisable::from_str(a)?),
        a if ProtocolLockEnable::from_str(a).is_ok() => eval(e, ProtocolLockEnable::from_str(a)?),
        a if ProtocolLockState::from_str(a).is_ok() => eval(e, ProtocolLockState::from_str(a)?),
        a if Reading::from_str(a).is_ok() => eval(e, Reading::from_str(a)?),
        a if Sleep::from_str(a).is_ok() => eval(e, Sleep::from_str(a)?),
        a if Status::from_str(a).is_ok() => eval(e, Status::from_str(a)?),
        _ => {
            error!("bad sensor command");
            Ok(format!("{:?}", ReplyStatus::Err))
        }
    }
}

// Parse the command-line arguments and execute.
fn evaluate_command_line() -> Result<()> {
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
    let sensor = ConductivitySensor::from_config(sensor_cfg)?;

    // initialize the socket.
    let socket = socket_from_config(&socket_cfg)?;

    // initialize the responder with the sensor and socket.
    let responder = ConductivityResponder::new(socket, sensor)?;

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
