#![recursion_limit = "1024"]
//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate benita;
extern crate chrono;
extern crate zmq;

use chrono::{DateTime, Local};
use benita::errors::*;

fn atoi(s: &str) -> i64 {
    s.parse().unwrap()
}

fn run() -> Result<()> {
    println!("Collecting updates from weather server...");

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());

    let filter = "temp_uuid";
    assert!(subscriber.set_subscribe(filter.as_bytes()).is_ok());

    let mut total_temp = 0;

    loop {
        for _ in 0 .. 6 {
            let string = subscriber.recv_string(0).unwrap().unwrap();
            let chks: Vec<&str> = string.split(' ').collect();
            let (_uuid, datetime, temperature) = (chks[0], chks[1], atoi(&chks[2]));
            let dt = datetime.parse::<DateTime<Local>>();
            println!("{:?} {}", dt, temperature);
            total_temp += temperature;
        }

        println!("Average temperature for '{}' was {}°C", "temp_uuid", (total_temp / 6));
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
