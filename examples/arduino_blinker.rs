extern crate benita;
extern crate i2cdev;

use benita::{I2cCommand, I2cSlave, SlaveDevice};
use i2cdev::linux::LinuxI2CError;

use std::env;

const ARDUINO_SLAVE_ADDR: u16 = 0x08;
const I2CBUS_ID: u8 = 1;

enum SlaveCommand {
    BlinkerOn,
    BlinkerOff,
}

impl I2cCommand for SlaveCommand {
    fn parse(&self) -> Vec<u8> {
        match self {
            &SlaveCommand::BlinkerOn => vec![0x01],
            &SlaveCommand::BlinkerOff => vec![0x00],
        }
    }
}

struct BlinkerService {
    device: SlaveDevice
}

impl BlinkerService {
    fn new(device: SlaveDevice) -> Self {
        BlinkerService { device }
    }
    fn on(&self) -> Result<(), LinuxI2CError> {
        println!("Sending: Blink on");
        self.device.send(SlaveCommand::BlinkerOn)
    }
    fn off(&self) -> Result<(), LinuxI2CError>{
        println!("Sending: Blink off");
        self.device.send(SlaveCommand::BlinkerOff)
    }
}

/// This is the main program. It takes one argument: if it is 'on', the I2C
/// slave will be commanded to turn on blinking on the LED.
/// Anything else will turn it off.
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let arduino = SlaveDevice::new(I2CBUS_ID, ARDUINO_SLAVE_ADDR);
    let blinker = BlinkerService::new(arduino);
    match &command[..] {
        "on" => blinker.on().unwrap(),
        "off" | _ => blinker.off().unwrap(),
    };
    ::std::process::exit(0);
}
