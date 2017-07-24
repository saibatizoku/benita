#![recursion_limit = "1024"]
//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate chrono;
extern crate ezo_rtd;
extern crate i2cdev;
extern crate zmq;

use std::thread;
use std::time::Duration;

use ezo_rtd::errors::*;
use ezo_rtd::command as rtd_command;
use ezo_rtd::response as rtd_response;
use rtd_command::Command;
use rtd_response::Temperature;

use chrono::{DateTime, Utc};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65
const PUB_CHANNEL: &'static str = "temperature-0123456789abcdef";

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    loop {
        // We take a temperature reading (around 900ms).
        let temperature = rtd_command::ReadingWithScale.run(&mut dev)?;
        let (temp_float, temp_scale) = match temperature {
            Temperature::Celsius(t) => (t, "Celsius"),
            Temperature::Kelvin(t) => (t, "Kelvin"),
            Temperature::Fahrenheit(t) => (t, "Fahrenheit"),
        };

        // We immediately put the chip to sleep.
        let _sleep = rtd_command::Sleep.run(&mut dev)?;

        // We print out the result
        let dt: DateTime<Utc> = Utc::now();
        let update = format!("{} {:?} {:.*}, {}", PUB_CHANNEL, dt, 2, temp_float, temp_scale);
        publisher.send(&update.as_bytes(), 0).unwrap();
        println!("{}", &update);

        // put the thread to sleep for 10_000 - 900 ms = 9_100 ms.
        // The real delay will depend on your system's characteristics.
        thread::sleep(Duration::from_millis(9_100));
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
