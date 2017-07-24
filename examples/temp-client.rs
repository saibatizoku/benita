#![recursion_limit = "1024"]
//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate benita;
extern crate chrono;
extern crate zmq;

use chrono::{DateTime, Local};
use benita::errors::*;

const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";

fn atof(s: &str) -> f64 {
    s.parse().unwrap()
}

fn run() -> Result<()> {
    println!("Collecting updates from weather server...");

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://192.168.16.123:5556").is_ok());

    assert!(subscriber.set_subscribe(SUB_CHANNEL.as_bytes()).is_ok());

    let mut total_temp = 0f64;

    loop {
        for _ in 0 .. 6 {
            let string = subscriber.recv_string(0).unwrap().unwrap();
            let chks: Vec<&str> = string.split(' ').collect();
            let (_uuid, datetime, temperature, scale) = (chks[0], chks[1], atof(&chks[2]), chks[3]);
            let dt = datetime.parse::<DateTime<Local>>().unwrap();
            println!("{:?} {} {}", dt, temperature, scale);
            total_temp += temperature;
        }

        println!("Average temperature for '{}' was {}", "temp_uuid", (total_temp / 6.0));
        total_temp = 0f64;
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
