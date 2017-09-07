//! Allows for remote command of the EC EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
extern crate ezo_ec;
extern crate i2cdev;
extern crate neuras;

use std::thread;
use std::time::Duration;

use benita::errors::{Result, ResultExt};
use clap::{App, Arg};
use ezo_ec::command as ec_command;
use ezo_ec::response as ec_response;
use ec_command::Command;
use ec_response::OutputStringStatus;
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

#[derive(Debug, Clone, Copy, PartialEq)]
enum PossibleCommand {
    // 'T,n' command, where n is a temperature float/int
    Calibrate(f64),
    // 'O,?' command
    GetParams,
    // 'R'
    Read,
    // 'SLEEP' command
    Sleep,
    // command not recognized
    NotRecognized,
}

fn parse_command(cmd_str: &str) -> PossibleCommand {
    match cmd_str {
        "read" => PossibleCommand::Read,
        a if cmd_str.starts_with("calibrate ") => {
            let rest = a.get(10..).unwrap();
            let temp = rest.parse().unwrap();
            PossibleCommand::Calibrate(temp)
        }
        "get_params" => PossibleCommand::GetParams,
        "sleep" => PossibleCommand::Sleep,
        _ => PossibleCommand::NotRecognized,
    }
}

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

    run(&rep_url)?;

    // Never reach this line...
    Ok(())
}

fn run(rep_url: &str) -> Result<()> {
    // We initialize our I2C device connection.
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    // We start our ZMQ context.
    let context = neuras::utils::create_context();
    // We configure our socket as REP, for accepting requests
    // and providing REsPonses.
    let responder = neuras::utils::zmq_rep(&context)?;
    // We bind our socket to REP_URL.
    assert!(responder.bind(rep_url).is_ok());
    // We initialize our ZMQ message. It will be reused throughout.
    let mut msg = neuras::utils::create_message()?;

    // This is the main loop, it will run for as long as the program runs.
    loop {
        // We start by recieving the command request from the client.
        responder.recv(&mut msg, 0).unwrap();

        // The command as a str.
        let msg_str = msg.as_str().unwrap();

        // Parse and process the command.
        let command_response = match parse_command(msg_str) {
            PossibleCommand::Calibrate(temp) => {
                let _calibrate = ec_command::TemperatureCompensation(temp).run(&mut dev)?;
                format!("Compensated Temperature: {}", temp)
            }
            PossibleCommand::NotRecognized => "Unknown command".to_string(),
            PossibleCommand::GetParams => {
                let output_state: OutputStringStatus = ec_command::OutputState.run(&mut dev)?;
                output_state.to_string()
            }
            PossibleCommand::Read => {
                let sensor_output = ec_command::Reading.run(&mut dev)?;
                format!("{:?}", sensor_output)
            }
            PossibleCommand::Sleep => {
                let _sleep = ec_command::Sleep.run(&mut dev)?;
                "Sleeping".to_string()
            }
        };

        // Send response to the client.
        responder.send(command_response.as_bytes(), 0).unwrap();

        // No work left, so we sleep.
        thread::sleep(Duration::from_millis(1));
    }
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
