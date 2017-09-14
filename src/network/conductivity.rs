//! Executable programs for handling sensors.
use std::thread;
use std::time::Duration;

use config::SensorServiceConfig;
use errors::*;
use neuras;

use chrono::{DateTime, Local};

/// REP command-set.
///
/// 'T,n' command, where n is a temperature float/int
/// 'O,?' command
/// 'R'
/// 'SLEEP' command
/// command not recognized
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum REPCommand {
    Calibrate(f64),
    GetParams,
    Read,
    Sleep,
}

impl REPCommand {
    pub fn parse(cmd_str: &str) -> Result<REPCommand> {
        let cmd = match cmd_str {
            "read" => REPCommand::Read,
            a if cmd_str.starts_with("calibrate ") => {
                let rest = a.get(10..).unwrap();
                let temp = rest.parse().unwrap();
                REPCommand::Calibrate(temp)
            }
            "get_params" => REPCommand::GetParams,
            "sleep" => REPCommand::Sleep,
            _ => return Err(ErrorKind::CommandParse.into()),
        };
        Ok(cmd)
    }
}

fn atof(s: &str) -> f64 {
    s.parse().unwrap()
}

fn parse_calibration_value_msg(sub_msg: &str) -> Result<(String, DateTime<Local>, f64, String)> {
    let mut split = sub_msg.split(' ');
    // The first string is the UUID of the message source.
    let uuid = match split.next() {
        Some(_uuid) => _uuid.to_string(),
        _ => {
            println!("No valid UUID found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };
    // The second string is the UTC datetime. We parse it as a local datetime.
    let dt = match split.next() {
        Some(date_n_time) => date_n_time.parse::<DateTime<Local>>().unwrap(),
        _ => {
            println!("NO valid date-time found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };
    // The third string is the temperature value of the sample.
    let temperature = match split.next() {
        Some(temp) => atof(&temp),
        _ => {
            println!("NO valid date-time found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };
    // The fourth string is the temperature scale of the sample.
    let scale = match split.next() {
        Some(_scale) => _scale.to_string(),
        _ => {
            println!("NO valid temperature scale found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };

    Ok((uuid, dt, temperature, scale))
}

#[allow(dead_code)]
pub fn calibrated_sampler(config: SensorServiceConfig) -> Result<()> {
    // Create ZMQ context
    let context = neuras::utils::create_context();

    // Create ZMQ sockets
    let subscriber = neuras::utils::zmq_sub(&context)?;
    let req_ec = neuras::utils::zmq_req(&context)?;
    let req_ph = neuras::utils::zmq_req(&context)?;

    let _connect_sub = neuras::utils::connect_socket(&subscriber, config.pub_url)?;
    let _subscribe = neuras::utils::subscribe_client(&subscriber, config.channel)?;

    let _connect_ec = neuras::utils::connect_socket(&req_ec, config.rep_ec_url)?;
    let _connect_ph = neuras::utils::connect_socket(&req_ph, config.rep_ph_url)?;

    // Continued program logic goes here...
    println!("Collecting updates from weather server...");

    let mut samples = 1;
    let mut total_temp = 0f64;

    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        let sub_str = subscriber.recv_string(0).unwrap().unwrap();

        let (uuid, dt, temperature, scale) = parse_calibration_value_msg(&sub_str)?;
        println!(
            "{} {} {}",
            dt.format("%F %T %z").to_string(),
            temperature,
            scale
        );

        total_temp += temperature;

        if samples == 6 {
            let avg_temp = total_temp / 6.0;
            println!("UUID: {} AVG: {:.*} {}", uuid, 3, avg_temp, scale);
            println!("Calibrating EC: {}", dt.format("%F %T %z").to_string());

            let mut msg = neuras::utils::create_message()?;

            // PH
            let _send = req_ph.send("read".as_bytes(), 0).unwrap();
            let _recv = req_ph.recv(&mut msg, 0).unwrap();
            println!("pH {}", msg.as_str().unwrap());

            let _send = req_ph.send("sleep".as_bytes(), 0).unwrap();
            let _recv = req_ph.recv(&mut msg, 0).unwrap();

            // EC
            let calibrate = format!("calibrate {:.*}", 3, avg_temp);
            let _send = req_ec.send(calibrate.as_bytes(), 0).unwrap();
            let _recv = req_ec.recv(&mut msg, 0).unwrap();
            println!("{}", msg.as_str().unwrap());

            let _send = req_ec.send("get_params".as_bytes(), 0).unwrap();
            let _recv = req_ec.recv(&mut msg, 0).unwrap();
            println!("{}", msg.as_str().unwrap());

            let _send = req_ec.send("read".as_bytes(), 0).unwrap();
            let _recv = req_ec.recv(&mut msg, 0).unwrap();
            println!("{}", msg.as_str().unwrap());

            let _send = req_ec.send("sleep".as_bytes(), 0).unwrap();
            let _recv = req_ec.recv(&mut msg, 0).unwrap();

            total_temp = 0f64;
            samples = 1;
        } else {
            samples += 1;
        }

        // No work left, so we sleep.
        thread::sleep(Duration::from_millis(1));
    }
    // Never reach this line...
}
