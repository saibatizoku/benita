//! Take a reading every 10 seconds from the RTD EZO chip, and publish the
//! data with a UUID as the topic.
//!
//! This server binds to the `PUB_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
extern crate ezo_rtd;
extern crate i2cdev;
extern crate neuras;

use std::thread;
use std::time::Duration;

use benita::errors::*;
use clap::{App, Arg};
use ezo_rtd::command as rtd_command;
use ezo_rtd::response as rtd_response;
use neuras::{create_context, zmq_pub};
use rtd_command::Command;
use rtd_response::SensorReading;

use chrono::{DateTime, Utc};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 101; // could be specified as 0x65
const PUB_CHANNEL: &'static str = "temperature-0123456789abcdef";

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-temperature-pub")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. A publication service for temperature data.")
        .arg(Arg::with_name("pub-url")
                 .short("p")
                 .long("pub-url")
                 .value_name("PUB_URL")
                 .help("Sets the url for the publication server")
                 .takes_value(true)
                 .index(1)
                 .required(true))
        .get_matches();

    let mut pub_url = String::new();

    if let Some(c) = matches.value_of("pub-url") {
        pub_url = String::from(c);
    }

    run(&pub_url)?;

    // Never reach this line...
    Ok(())
}

fn run(pub_url: &str) -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    let context = create_context();
    let publisher = zmq_pub(&context)?;

    assert!(publisher.bind(pub_url).is_ok());

    loop {
        // We query the current temperature state of the sensor chip.
        let scale = rtd_command::ScaleState.run(&mut dev)?;

        // We take a temperature reading (around 600ms).
        let SensorReading(temperature) = rtd_command::Reading.run(&mut dev)?;

        // We immediately put the chip to sleep.
        let _sleep = rtd_command::Sleep.run(&mut dev)?;

        // We print out the result with the current ISO datetime.
        let dt: DateTime<Utc> = Utc::now();
        let update = format!("{} {:?} {:.*} {:?}", PUB_CHANNEL, dt, 3, temperature, scale);
        publisher.send(&update.as_bytes(), 0).unwrap();
        println!("{}", &update);

        // put the thread to sleep for 10_000 - 900 ms = 9_100 ms.
        // The real delay will depend on your system's characteristics.
        thread::sleep(Duration::from_millis(9_100));
    }
}

fn main() {
    if let Err(ref e) = parse_cli_arguments() {
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
