//! An example that takes readings from the RTD EZO chip in a loop.
//!

#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;
extern crate chrono;
extern crate neuras;
extern crate toml;
extern crate zmq;

use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

use benita::errors::*;
use benita::Config;
use clap::{App, Arg};
use chrono::{DateTime, Local};
use neuras::{zmq_req, zmq_sub, connect_client};


const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";

fn atof(s: &str) -> f64 {
    s.parse().unwrap()
}

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .arg(Arg::with_name("pub-server-url")
                 .short("p")
                 .long("pub-server")
                 .value_name("PUB_URL")
                 .help("Sets the url for the PUB server")
                 .takes_value(true)
                 .index(1)
                 .conflicts_with_all(&["config"]))
        .arg(Arg::with_name("rep-server-url")
                 .short("r")
                 .long("rep-server")
                 .value_name("REP_URL")
                 .help("Sets the url for the REP server")
                 .takes_value(true)
                 .index(2)
                 .conflicts_with_all(&["config"]))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .multiple(true)
                 .help("Turn debugging information on"))
        .get_matches();

    let mut input = String::new();
    let mut config = Config::default();
    config.channel = SUB_CHANNEL;

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", &c);
        let _read = File::open(&c)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();

        config = Config::from_str(&input)?;
    } else {
        if let Some(pub_url) = matches.value_of("pub-server-url") {
            config.pub_url = pub_url;
        }

        if let Some(rep_url) = matches.value_of("rep-server-url") {
            config.rep_url = rep_url;
        }
    }

    let context = zmq::Context::new();
    let subscriber = zmq_sub(&context)?;
    let requester = zmq_req(&context)?;

    let _connect_sub = connect_client(&subscriber, config.pub_url)?;
    let _subscribe = subscriber.set_subscribe(config.channel.as_bytes())?;

    let _connect_req = connect_client(&requester, config.rep_url)?;

    // Continued program logic goes here...
    let _r = run_calibrated_ec_service(&subscriber, &requester)?;

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

fn send_ec_request(requester: &zmq::Socket, avg_temp: f64) -> Result<()> {
    let mut msg = zmq::Message::new().unwrap();

    let calibrate = format!("calibrate {:.*}", 3, avg_temp);
    let _send = requester.send(calibrate.as_bytes(), 0).unwrap();
    let _recv = requester.recv(&mut msg, 0).unwrap();
    println!("{}", msg.as_str().unwrap());

    let _send = requester.send("get_params".as_bytes(), 0).unwrap();
    let _recv = requester.recv(&mut msg, 0).unwrap();
    println!("{}", msg.as_str().unwrap());

    let _send = requester.send("read".as_bytes(), 0).unwrap();
    let _recv = requester.recv(&mut msg, 0).unwrap();
    println!("{}", msg.as_str().unwrap());

    let _send = requester.send("sleep".as_bytes(), 0).unwrap();
    let _recv = requester.recv(&mut msg, 0).unwrap();

    Ok(())
}

fn run_calibrated_ec_service(subscriber: &zmq::Socket, requester: &zmq::Socket) -> Result<()> {
    println!("Collecting updates from weather server...");

    let mut samples = 1;
    let mut total_temp = 0f64;

    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        let sub_str = subscriber.recv_string(0).unwrap().unwrap();

        let (uuid, dt, temperature, scale) = parse_sub_str(&sub_str)?;
        println!("{} {} {}",
                 dt.format("%F %T %z").to_string(),
                 temperature,
                 scale);

        total_temp += temperature;

        if samples == 6 {
            let avg = total_temp / 6.0;
            println!("UUID: {} AVG: {:.*} {}", uuid, 3, avg, scale);

            println!("Calibrating EC: {}", dt.format("%F %T %z").to_string());
            let _req = send_ec_request(requester, avg)?;

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
