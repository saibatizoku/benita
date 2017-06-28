#![recursion_limit = "1024"]
extern crate benita;
extern crate i2cdev;

use benita::{I2cCommand, I2cSlave, SlaveDevice};
use benita::errors::*;
use i2cdev::linux::LinuxI2CError;

use std::env;

const ARDUINO_SLAVE_ADDR: u16 = 0x08;
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
    device: SlaveDevice,
}

impl BlinkerService {
    fn new(device: SlaveDevice) -> Self {
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

/// This is the main program. It takes one argument: if it is 'on', the I2C
/// slave will be commanded to turn on blinking on the LED. 'off' will turn
/// the blinking off.
///
/// Anything else will exit normally, printing a message.
///
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let arduino = SlaveDevice::new(I2CBUS_ID, ARDUINO_SLAVE_ADDR);
    let blinker = BlinkerService::new(arduino.clone());
    match &command[..] {
        "on"  => blinker.on().unwrap(),
        "off" => blinker.off().unwrap(),
        _     => println!("'{}' doesn't exist. Use 'on' or 'off'.", command),
    };
    ::std::process::exit(0);
}
