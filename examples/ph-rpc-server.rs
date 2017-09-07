//! Allows for remote command of the PH EZO chip, exposing a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
extern crate ezo_ph;
extern crate i2cdev;
extern crate neuras;

use std::thread;
use std::time::Duration;

use benita::errors::{Result, ResultExt};
use clap::{App, Arg};
use ezo_ph::command as ph_command;
use ezo_ph::response as ph_response;
use ph_command::Command;
use ph_response::SensorReading;
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 99; // could be specified as 0x63

#[derive(Debug, Clone, Copy, PartialEq)]
enum PossibleCommand {
    // 'T,n' command, where n is a temperature float/int
    Calibrate(f64),
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
        "sleep" => PossibleCommand::Sleep,
        _ => PossibleCommand::NotRecognized,
    }
}

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-ph-rep")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. A response service for pH data.")
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
                let _calibrate = ph_command::TemperatureCompensation(temp).run(&mut dev)?;
                format!("Compensated Temperature: {}", temp)
            }
            PossibleCommand::Read => {
                let SensorReading(sensor_output) = ph_command::Reading.run(&mut dev)?;
                format!("{}", sensor_output)
            }
            PossibleCommand::Sleep => {
                let _sleep = ph_command::Sleep.run(&mut dev)?;
                "Sleeping".to_string()
            }
            PossibleCommand::NotRecognized => "Unknown command".to_string(),
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
