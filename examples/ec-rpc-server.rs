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
    GetParams,
    NotRecognized,
    Read,
    Sleep,
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
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5557").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        let msg_str = msg.as_str().unwrap();
        println!("Received {}", msg_str);
        let _command_response = match parse_command(msg_str) {
            PossibleCommand::NotRecognized => {
                "Unknown command".to_string()
            }
            PossibleCommand::GetParams => {
                let output_state = ec_command::OutputState.run(&mut dev)?;
                format!("{}", output_state.to_string())
            }
            PossibleCommand::Read => {
                let sensor_output = ec_command::Reading.run(&mut dev)?;
                format!("{:?}", sensor_output)
            }
            PossibleCommand::Sleep => {
                "Sleeping".to_string()
            }
        };
        //thread::sleep(Duration::from_millis(1));
        responder.send(_command_response.as_bytes(), 0).unwrap();
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
