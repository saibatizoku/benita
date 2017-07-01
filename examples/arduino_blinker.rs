#![recursion_limit = "1024"]
extern crate benita;
extern crate i2cdev;

use benita::{I2cCommand, I2cSensing, SensingDevice};
use benita::errors::*;

use std::env;

const ARDUINO_SENSING_ADDR: u16 = 0x08;
const I2CBUS_ID: u8 = 1;

enum BlinkerCommand {
    On,
    Off,
}

impl I2cCommand for BlinkerCommand {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            &BlinkerCommand::On => vec![0x01],
            &BlinkerCommand::Off => vec![0x00],
        }
    }
    fn to_string(&self) -> String {
        format!("{:?}", self.to_bytes())
    }
}

struct BlinkerService {
    device: SensingDevice,
}

impl BlinkerService {
    fn new(device: SensingDevice) -> Self {
        BlinkerService { device }
    }
    fn on(&self) -> Result<()> {
        println!("Sending: Blink on");
        self.device.send(BlinkerCommand::On)
    }
    fn off(&self) -> Result<()> {
        println!("Sending: Blink off");
        self.device.send(BlinkerCommand::Off)
    }
}

/// This is the main program, it executes `run_service` with error-chain.
fn main() {
    if let Err(ref e) = run_service() {
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

/// This is the main service. It takes one argument: if it is 'on', the I2C
/// sensing device will be commanded to turn on blinking on the LED. 'off' will
/// turn the blinking off.
///
/// Anything else will exit normally, printing a message.
///
fn run_service() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let arduino = SensingDevice::new(I2CBUS_ID, ARDUINO_SENSING_ADDR);
    let blinker = BlinkerService::new(arduino.clone());
    match &command[..] {
        "on"  => blinker.on().unwrap(),
        "off" => blinker.off().unwrap(),
        _     => println!("'{}' doesn't exist. Use 'on' or 'off'.", command),
    };
    Ok(())
}
