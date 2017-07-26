//! Allows for remote command of the RTD EZO chip, exposing a limited API.
//!
//! This server binds to `tcp://*:5557`.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate ezo_ec;
extern crate i2cdev;
extern crate zmq;

use ezo_ec::errors::*;
use ezo_ec::command as ec_command;
use ezo_ec::response as ec_response;
use ec_command::Command;
use chrono::{DateTime, Utc};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 100; // could be specified as 0x64

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PossibleCommand {
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
        "get_params" => PossibleCommand::GetParams,
        "sleep" => PossibleCommand::Sleep,
        _ => PossibleCommand::NotRecognized,
    }
}

fn run() -> Result<()> {
    // We initialize our I2C device connection.
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    // We start our ZMQ context.
    let context = zmq::Context::new();
    // We configure our socket as REP, for accepting requests
    // and providing REsPonses.
    let responder = context.socket(zmq::REP).unwrap();
    // We bind our socket to local port 5557, using TCP.
    assert!(responder.bind("tcp://*:5557").is_ok());
    // We initialize our ZMQ message. It will be reused throughout.
    let mut msg = zmq::Message::new().unwrap();

    // This is the main loop, it will run for as long as the program runs.
    loop {
        // We start by recieving the command request from the client.
        responder.recv(&mut msg, 0).unwrap();

        // The command as a str.
        let msg_str = msg.as_str().unwrap();

        // Parse and process the command.
        let _command_response = match parse_command(msg_str) {
            PossibleCommand::NotRecognized => {
                "Unknown command".to_string()
            }
            PossibleCommand::GetParams => {
                let output_state = ec_command::OutputState.run(&mut dev)?;
                output_state.to_string()
            }
            PossibleCommand::Read => {
                let sensor_output = ec_command::Reading.run(&mut dev)?;
                format!("{:?}", sensor_output)
            }
            PossibleCommand::Sleep => {
                "Sleeping".to_string()
            }
        };

        // Send response to the client.
        responder.send(_command_response.as_bytes(), 0).unwrap();

        // No work left, so we sleep.
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    if let Err(ref e) = run() {
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
