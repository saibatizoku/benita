//! An example that takes readings from the RTD EZO chip in a loop.
//!
extern crate benita;
extern crate chrono;
extern crate clap;
extern crate neuras;

use std::thread;
use std::time::Duration;

use benita::errors::{ErrorKind, Result};
use chrono::{DateTime, Local};
use clap::{App, Arg};
use neuras::utils::{connect_socket, create_context, subscribe_client, zmq_sub};

const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";

fn atof(s: &str) -> f64 {
    s.parse().unwrap()
}

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-subscriber")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. Subscriber client.")
        .arg(
            Arg::with_name("pub-url")
                .short("b")
                .long("pub")
                .value_name("PUB_URL")
                .help("Sets the url for the PUB server")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("channel")
                .short("c")
                .long("channel")
                .value_name("CHANNEL")
                .help("Sets the subscription channel")
                .takes_value(true)
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .multiple(true)
                .help("Turn debugging information on"),
        )
        .get_matches();

    let pub_url = match matches.value_of("pub-url") {
        Some(puburl) => puburl,
        _ => return Err(ErrorKind::ConfigParse.into()),
    };

    let channel = match matches.value_of("channel") {
        Some(ch) => ch,
        _ => SUB_CHANNEL,
    };

    let _run = run_subscriber(&pub_url, &channel)?;

    // Never reach this line...
    Ok(())
}

fn parse_sub_str(sub_str: &str) -> Result<(String, DateTime<Local>, f64, String)> {
    let mut split = sub_str.split(' ');

    // The first string is the UUID of the message source.
    let uuid = match split.next() {
        Some(_uuid) => _uuid.to_string(),
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
        Some(_scale) => _scale.to_string(),
        _ => {
            println!("NO valid temperature scale found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };

    Ok((uuid, dt, temperature, scale))
}

fn run_subscriber(pub_url: &str, channel: &str) -> Result<()> {
    println!("Collecting updates from weather server...");

    let context = create_context();
    let subscriber = zmq_sub(&context)?;
    let _connect = connect_socket(&subscriber, pub_url)?;

    let _subscribe = subscribe_client(&subscriber, channel)?;

    let mut samples = 0;
    let mut total_temp = 0f64;

    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        let sub_str = subscriber.recv_string(0).unwrap().unwrap();

        let (uuid, dt, temperature, scale) = parse_sub_str(&sub_str)?;
        println!(
            "{} {} {}",
            dt.format("%F %T %z").to_string(),
            temperature,
            scale
        );

        total_temp += temperature;

        if samples == 6 {
            let avg = total_temp / 6.0;
            println!("UUID: {} AVG: {:.*} {}", uuid, 3, avg, scale);

            total_temp = 0f64;
            samples = 1;
        } else {
            samples += 1;
        }

        // No work left, so we sleep.
        thread::sleep(Duration::from_millis(1));
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
