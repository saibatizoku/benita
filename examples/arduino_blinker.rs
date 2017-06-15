extern crate benita;
extern crate i2cdev;

use benita::I2cCommand;
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use std::env;

const ARDUINO_SLAVE_ADDR: u16 = 0x08;
const I2CBUS_ID: u8 = 1;

enum ArduinoCommand {
    BlinkerOn,
    BlinkerOff,
}

struct ArduinoDevice {
    bus: u8,
    address: u16,
}

impl ArduinoDevice {
    pub fn new(bus: u8, address: u16) -> ArduinoDevice {
        ArduinoDevice { bus, address }
    }
    pub fn send(&self, cmd: ArduinoCommand) -> Result<(), LinuxI2CError> {
        let bus = format!("/dev/i2c-{}", self.bus);
        let mut dev = try!(LinuxI2CDevice::new(bus, self.address));
        dev.smbus_write_byte(cmd.parse())
    }
}

impl I2cCommand for ArduinoCommand {
    fn parse(&self) -> u8 {
        match self {
            &ArduinoCommand::BlinkerOn => 0x01,
            &ArduinoCommand::BlinkerOff => 0x00,
        }
    }
}

fn blinker_on(dev: &ArduinoDevice) {
    println!("Sending: Blink on");
    dev.send(ArduinoCommand::BlinkerOn).unwrap();
}


fn blinker_off(dev: &ArduinoDevice) {
    println!("Sending: Blink off");
    dev.send(ArduinoCommand::BlinkerOff).unwrap();
}

/// This is the main program. It takes one argument: if it is 'on', the I2C
/// slave will be commanded to turn on blinking on the LED.
/// Anything else will turn it off.
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let device = ArduinoDevice::new(I2CBUS_ID, ARDUINO_SLAVE_ADDR);
    match &command[..] {
        "on" => blinker_on(&device),
        _ => blinker_off(&device),
    };
    ::std::process::exit(0);
}
