//! Network services for sensors.
use std::thread;
use std::time::Duration;

use config::SensorServiceConfig;
use errors::*;
use network::conductivity::{ConductivityClient, REPCommand};
use network::ph::PhClient;
use sensors::conductivity::ConductivitySensor;

use neuras;
use neuras::utils::bind_socket;
use chrono::{DateTime, Local};

/// The device's UUID.
type DeviceUuid = String;
/// The Temperature Scale.
type TemperatureScale = String;

/// Conductivity RESPONSE server. Handles requests from the given URL, and
/// directly interfacing with the sensor at the `i2c_path`, located at
/// the `i2c_address` location.
pub fn run_conductivity_server(rep_url: &str, i2c_path: &str, i2c_address: u16) -> Result<()> {
    // We initialize our I2C device connection.
    let mut ec_sensor = ConductivitySensor::new(&i2c_path, i2c_address)
        .chain_err(|| "Could not open I2C device")?;

    // We start our ZMQ context.
    let context = neuras::utils::create_context();

    // We configure our socket as REP, for accepting requests
    // and providing REsPonses.
    let responder = neuras::utils::zmq_rep(&context)?;
    // We bind our socket to REP_URL.
    let _bind_socket = bind_socket(&responder, rep_url).chain_err(|| "problems binding to socket")?;
    // We initialize our ZMQ message. It will be reused throughout.
    let mut msg = neuras::utils::create_message()?;

    // This is the main loop, it will run for as long as the program runs.
    loop {
        // We start by recieving the command request from the client.
        responder.recv(&mut msg, 0).unwrap();

        // The command as a str.
        let msg_str = msg.as_str().unwrap();

        // Parse and process the command.
        let command_response = match REPCommand::parse(msg_str) {
            Ok(REPCommand::Calibrate(temp)) => match ec_sensor.set_compensation_temperature(temp) {
                Ok(_) => format!("temperature-compensation {}", temp),
                Err(e) => format!("error {}", e),
            },
            Ok(REPCommand::GetParams) => match ec_sensor.get_output_string_status() {
                Ok(output_state) => output_state.to_string(),
                Err(e) => format!("error {}", e),
            },
            Ok(REPCommand::Read) => match ec_sensor.get_reading() {
                Ok(sensor_output) => format!("{:?}", sensor_output),
                Err(e) => format!("error {}", e),
            },
            Ok(REPCommand::Sleep) => match ec_sensor.set_sleep() {
                Ok(_) => "sleeping".to_string(),
                Err(e) => format!("error {}", e),
            },
            // Respond with the given error
            Err(e) => format!("error {}", e),
        };

        // Send response to the client.
        responder.send(command_response.as_bytes(), 0).unwrap();

        // No work left, so we sleep.
        thread::sleep(Duration::from_millis(1));
    }
}

/// Run a calibrated sensor service.
///
/// This service samples the temperature at a given interval, using that value
/// to compensate the pH sensor and sample, and then doing the same with the
/// conductivity sensor.
#[allow(dead_code)]
pub fn run_calibrated_sampling_service(config: SensorServiceConfig) -> Result<()> {
    // Create ZMQ context
    let context = neuras::utils::create_context();

    // Setup network sockets:
    // subscriber SUB socket
    let subscriber = neuras::utils::zmq_sub(&context)?;
    // conductivity REQ socket
    let req_ec = neuras::utils::zmq_req(&context)?;
    // pH REQ socket
    let req_ph = neuras::utils::zmq_req(&context)?;

    // Connect and subscribe
    let _connect_sub = neuras::utils::connect_socket(&subscriber, config.pub_url)?;
    let _subscribe = neuras::utils::subscribe_client(&subscriber, config.channel)?;

    // Connect REQ sockets
    let _connect_ec = neuras::utils::connect_socket(&req_ec, config.rep_ec_url)?;
    let _connect_ph = neuras::utils::connect_socket(&req_ph, config.rep_ph_url)?;

    // This is the client that will send commands to the `Conductivity` sensor.
    let mut ec_client = ConductivityClient::new(req_ec)?;
    // This is the client that will send commands to the `pH` sensor.
    let mut ph_client = PhClient::new(req_ph)?;

    // Continued program logic goes here...
    println!("Collecting updates from weather server...");

    let mut samples = 1;
    let mut total_temp = 0f64;

    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        // Receive and parse the string from the subscription channel.
        let sub_str = subscriber.recv_string(0).unwrap().unwrap();
        let (uuid, dt, temperature, scale) = parse_calibration_value_msg(&sub_str)?;
        // Print it out to the screen
        // TODO: use logging to handle this
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

            // PH
            let read = ph_client.send_read()?;
            println!("pH {}", read);

            let sleep = ph_client.send_sleep()?;
            println!("{}", sleep);

            // EC
            let compensate = ec_client.send_compensate(avg_temp)?;
            println!("{}", compensate);

            let output_params = ec_client.get_output_params()?;
            println!("{}", output_params);

            let read = ec_client.send_read()?;
            println!("{}", read);

            let sleep = ec_client.send_sleep()?;
            println!("{}", sleep);

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

// simple atof conversion.
fn atof(s: &str) -> f64 {
    s.parse().unwrap()
}

// parse the subscription message as `(DeviceUuid, DateTime<Local>, f64, TemperatureScale)`.
//
// This message is provided by a networked publication service, and it contains
// a message sent from a device.
fn parse_calibration_value_msg(
    sub_msg: &str,
) -> Result<(DeviceUuid, DateTime<Local>, f64, TemperatureScale)> {
    let mut split = sub_msg.split(' ');
    // The first string is the UUID of the message source.
    let uuid: DeviceUuid = match split.next() {
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
    let scale: TemperatureScale = match split.next() {
        Some(_scale) => _scale.to_string(),
        _ => {
            println!("NO valid temperature scale found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };

    Ok((uuid, dt, temperature, scale))
}