extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use std::env;

const ARDUINO_SLAVE_ADDR: u16 = 0x08;

enum ArduinoCommand {
    BlinkerOn,
    BlinkerOff,
}

impl ArduinoCommand {
    fn parse(&self) -> u8 {
        match self {
            &ArduinoCommand::BlinkerOn => 0x01,
            &ArduinoCommand::BlinkerOff => 0x00,
        }
    }

    fn send(&self) -> Result<(), LinuxI2CError> {
        let i2cbus = format!("/dev/i2c-{}", 1);
        let mut dev = try!(LinuxI2CDevice::new(i2cbus, ARDUINO_SLAVE_ADDR));
        dev.smbus_write_byte(self.parse())
    }
}

fn blinker_on() {
    println!("Sending: Blink on");
    ArduinoCommand::BlinkerOn.send().unwrap();
}


fn blinker_off() {
    println!("Sending: Blink off");
    ArduinoCommand::BlinkerOff.send().unwrap();
}

/// This is the main program. It takes one argument: if it is 'on', the I2C
/// slave will be commanded to turn on blinking on the LED.
/// Anything else will turn it off.
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match &command[..] {
        "on" => blinker_on(),
        "off" => blinker_off(),
        _ => blinker_off(),
    };
    ::std::process::exit(0);
}
