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
    let mut cnt = 0;

    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        let string = subscriber.recv_string(0).unwrap().unwrap();
        let mut split = string.split(' ');

        // The first string is the UUID of the message source.
        let uuid = match split.next() {
            Some(_uuid) => _uuid,
            _ => {
                println!("No valid UUID found");
                return Err(ErrorKind::ResponseParse.into());
            }
        };

        let dt = match split.next() {
            Some(date_n_time) => date_n_time.parse::<DateTime<Local>>().unwrap(),
            _ => {
                println!("NO valid date-time found");
                return Err(ErrorKind::ResponseParse.into());
            }
        };

        let temperature = match split.next() {
            Some(temp) => atof(&temp),
            _ => {
                println!("NO valid date-time found");
                return Err(ErrorKind::ResponseParse.into());
            }
        };


        let scale = match split.next() {
            Some(_scale) => _scale,
            _ => {
                println!("NO valid temperature scale found");
                return Err(ErrorKind::ResponseParse.into());
            }
        };
        println!("{} {} {}", dt.format("%F %T %z").to_string(), temperature, scale);

        if cnt < 5 {
            total_temp += temperature;
            cnt += 1;
        } else {
            let avg = total_temp / 6.0;
            println!("UUID: {} AVG: {:.*} {}", uuid, 3, avg, scale);
            total_temp = 0f64;
            cnt = 0;
        }
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
