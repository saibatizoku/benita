//! Calibrated multi-sensor service.
use std::thread;
use std::time::Duration;

use conductivity::ConductivityAPI;
use conductivity::network::ConductivityRequester;
use config::SensorServiceConfig;
use errors::*;
use network::SocketReply;
use temperature::network::replies::{SensorReading, TemperatureScale};
use ph::PhAPI;
use ph::network::PhRequester;

use chrono::{DateTime, Local};
use neuras;

/// The device's UUID.
type DeviceUuid = String;

fn setup_subscriber_socket(context: &neuras::zmq::Context) -> Result<neuras::zmq::Socket> {
    let sub = neuras::utils::zmq_sub(context).chain_err(|| ErrorKind::SocketCreate)?;
    Ok(sub)
}

fn setup_requester_socket(context: &neuras::zmq::Context) -> Result<neuras::zmq::Socket> {
    let req = neuras::utils::zmq_req(context).chain_err(|| ErrorKind::SocketCreate)?;
    Ok(req)
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
    let subscriber = setup_subscriber_socket(&context)?;
    // conductivity REQ socket
    let req_ec = setup_requester_socket(&context)?;
    // pH REQ socket
    let req_ph = setup_requester_socket(&context)?;

    // Connect and subscribe
    let _connect_sub = neuras::utils::connect_socket(&subscriber, config.pub_url)?;
    let _subscribe = neuras::utils::subscribe_client(&subscriber, config.channel)?;

    // Connect REQ sockets
    let _connect_ec = neuras::utils::connect_socket(&req_ec, config.rep_ec_url)?;
    let _connect_ph = neuras::utils::connect_socket(&req_ph, config.rep_ph_url)?;

    // This is the client that will send commands to the `Conductivity` sensor.
    let conductivity_client = ConductivityRequester::new(req_ec)?;
    // This is the client that will send commands to the `pH` sensor.
    let ph_client = PhRequester::new(req_ph)?;

    // Continued program logic goes here...
    info!("Collecting updates from weather server...");

    let mut samples = 1;
    let mut total_temp = 0f64;

    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        // Receive and parse the string from the subscription channel.
        let sub_str = subscriber.recv_string(0).unwrap().unwrap();
        let (uuid, _dt, temperature, scale) = parse_calibration_value_msg(&sub_str)?;
        // Print it out to the screen
        // TODO: use logging to handle this
        info!(target: &uuid, "{} {}", temperature, scale);

        total_temp += temperature.0;

        let n = 2;
        if samples == n {
            let avg_temp = total_temp / n as f64;
            info!(
                target: &uuid,
                "temperature avg {:.*} {}",
                3,
                avg_temp,
                scale
            );

            // PH
            let compensate = ph_client.set_compensation(avg_temp)?;
            info!(
                target: &uuid,
                "compensate {:.*} {} {}",
                3,
                avg_temp,
                &scale,
                compensate
            );

            let read = ph_client.get_reading()?;
            info!(target: &uuid, "{} pH", read);

            let sleep = ph_client.set_sleep()?;
            info!(target: &uuid, "sleep {}", sleep);

            // EC
            let compensate = conductivity_client.set_compensation(avg_temp)?;
            info!(
                target: &uuid,
                "compensate {:.*} {} {}",
                3,
                avg_temp,
                &scale,
                compensate
            );

            let output_params = conductivity_client.get_output_params()?;

            let read = conductivity_client.get_reading()?;

            let _o = format!("{}", output_params);
            let _r = format!("{}", read);
            let _readings = _o.split(",")
                .zip(_r.split(","))
                .map(|(k, v)| format!("{} {}", v, k))
                .for_each(|s| info!(target: &uuid, "{}", s));

            let sleep = conductivity_client.set_sleep()?;
            info!(target: &uuid, "sleep {}", sleep);

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

// parse the subscription message as `(DeviceUuid, DateTime<Local>, SensorReading,
// TemperatureScale)`.
//
// This message is provided by a networked publication service, and it contains
// a message sent from a device.
fn parse_calibration_value_msg(
    sub_msg: &str,
) -> Result<(DeviceUuid, DateTime<Local>, SensorReading, TemperatureScale)> {
    let mut split = sub_msg.split(' ');
    // The first string is the UUID of the message source.
    let uuid: DeviceUuid = match split.next() {
        Some(_uuid) => _uuid.to_string(),
        _ => {
            error!("No valid UUID found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };
    // The second string is the UTC datetime. We parse it as a local datetime.
    let dt = match split.next() {
        Some(date_n_time) => date_n_time.parse::<DateTime<Local>>().unwrap(),
        _ => {
            error!("NO valid date-time found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };
    // The third string is the temperature value of the sample.
    let temperature = match split.next() {
        Some(temp) => SensorReading::parse_response(&temp)?,
        _ => {
            error!("NO valid date-time found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };
    // The fourth string is the temperature scale of the sample.
    let scale: TemperatureScale = match split.next() {
        Some(scale) => TemperatureScale::parse_response(&scale)?,
        _ => {
            error!("NO valid temperature scale found");
            return Err(ErrorKind::ResponseParse.into());
        }
    };

    Ok((uuid, dt, temperature, scale))
}
