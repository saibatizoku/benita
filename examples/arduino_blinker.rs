extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use std::env;

const ARDUINO_SLAVE_ADDR: u16 = 0x08;

enum ArduinoCommand {
    BlinkerOn,
    BlinkerOff,
}

fn i2c_master_send(command: ArduinoCommand) -> Result<(), LinuxI2CError> {
    let cmd: u8 = match command {
        ArduinoCommand::BlinkerOn => {
            println!("Sending: Blink on");
            0x01
        },
        ArduinoCommand::BlinkerOff => {
            println!("Sending: Blink off");
            0x00
        },
    };
    let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-1", ARDUINO_SLAVE_ADDR));
    dev.smbus_write_byte(cmd).unwrap();
    Ok(())
}

/// This is the main program. It takes one argument: if it is 'on', the I2C
/// slave will be commanded to turn on blinking on the LED.
/// Anything else will turn it off.
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let i2c_cmd = match &command[..] {
        "on" => { ArduinoCommand::BlinkerOn },
        "off" => { ArduinoCommand::BlinkerOff },
        _ => { ArduinoCommand::BlinkerOff },
    };
    i2c_master_send(i2c_cmd).unwrap();
    ::std::process::exit(1);
}
