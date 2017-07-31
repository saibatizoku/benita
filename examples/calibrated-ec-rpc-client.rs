#![recursion_limit = "1024"]
//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate benita;
extern crate chrono;
extern crate zmq;

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Local};
use benita::errors::*;
use benita::neuras::{zmq_req, zmq_sub, connect_client};

const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";

fn atof(s: &str) -> f64 {
    s.parse().unwrap()
}

fn run() -> Result<()> {
    println!("Collecting updates from weather server...");

    let context = zmq::Context::new();
    let subscriber = zmq_sub(&context)?;
    let requester = zmq_req(&context)?;

    let _connect_sub = connect_client(&subscriber, "tcp://localhost:5558")?;
    let _connect_req = connect_client(&requester, "tcp://192.168.16.123:5557")?;

    assert!(subscriber.set_subscribe(SUB_CHANNEL.as_bytes()).is_ok());

    let mut total_temp = 0f64;
    let mut cnt = 0;
    let mut msg = zmq::Message::new().unwrap();

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

        if cnt < 4 {
            total_temp += temperature;
            cnt += 1;
        } else {
            let avg = total_temp / 4.0;
            println!("UUID: {} AVG: {:.*} {}", uuid, 3, avg, scale);
            {
                println!("Requesting Conductivity data: {}", dt.format("%F %T %z").to_string());
                let calibrate = format!("calibrate {:.*}", 2, avg);
                requester.send(calibrate.as_bytes(), 0).unwrap();
                requester.recv(&mut msg, 0).unwrap();
                println!("{}", msg.as_str().unwrap());

                requester.send("get_params".as_bytes(), 0).unwrap();
                requester.recv(&mut msg, 0).unwrap();
                println!("{}", msg.as_str().unwrap());

                requester.send("read".as_bytes(), 0).unwrap();
                requester.recv(&mut msg, 0).unwrap();
                println!("{}", msg.as_str().unwrap());

                requester.send("sleep".as_bytes(), 0).unwrap();
                requester.recv(&mut msg, 0).unwrap();
                println!("{}", msg.as_str().unwrap());
            }
            cnt = 0;
            total_temp = 0f64;
        }
        // No work left, so we sleep.
        thread::sleep(Duration::from_millis(1));
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
