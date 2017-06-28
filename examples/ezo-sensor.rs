#![recursion_limit = "1024"]
extern crate benita;
extern crate i2cdev;

use benita::errors::*;
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

use std::ascii::AsciiExt;
use std::thread;
use std::time::Duration;

const DEVICE_ID: u8 = 1;
const EZO_SLAVE_ADDR: u16 = 101; // could be specified as 0x65

// real code should probably not use unwrap()
fn i2cfun(cmd: &str, delay: u64) -> Result<()> {
    let device_path = format!("/dev/i2c-{}", DEVICE_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SLAVE_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    println!("I2C Device opened at {}", &device_path);

    println!("Sending command: '{}'", cmd.to_ascii_uppercase());
    dev.write(cmd.as_bytes()).chain_err(|| "Command could not be sent")?;
    println!("I2C Command sent");

    thread::sleep(Duration::from_millis(delay));
    let mut buf: [u8; 24] = [0; 24];
    dev.read(&mut buf).unwrap();
    match buf[0] {
        255 => println!("No data expected."),
        254 => println!("Pending"),
        2   => println!("Error"),
        1   => {
            println!("Success");
            if let Some(eol) = buf.into_iter().position(|&x| x == 0) {
                let data: String = buf[1..eol].into_iter().map(|c| {
                    (*c & !0x80) as char
                }).collect();
                println!("Response: {}", data);
            } else {
                println!("Reading: {:?}", String::from_utf8(Vec::from(&buf[1..])).unwrap());
            }
        },
        _ => println!("No response"),
    };
    Ok(())
}

/// This is the main program, it executes `run_command` with error-chain.
fn main() {
    if let Err(ref e) = run_command() {
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

/// This is the main code. It sends the command `cmd` and waits `delay` millis
/// before reading the response.
fn run_command() -> Result<()> {
    let cmd = "r\0";
    match i2cfun(&cmd, 600) {
        Ok(_) => println!("I2C command sent; Response processed"),
        Err(e) => println!("Bad luck, I2C command failed: {:?}", e),
    };
    Ok(())
}
