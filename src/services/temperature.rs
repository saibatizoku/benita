//! Network services for Temperature sensors.

use cli::temperature::TemperatureCommandApp;
use config::{SensorConfig, SocketConfig};
use devices::temperature::TemperatureSensor;
use errors::*;
use network::common::{Endpoint, SocketReply};
use network::temperature::TemperatureResponder;
use utilities::{atof, create_and_bind_responder};

use clap::ArgMatches;

use std::thread;
use std::time::Duration;


// Temperature sensor responder service.
sensor_responder_service! {
    "Temperature sensor responder service.",
    TemperatureSensorService: {
        TemperatureSensor, TemperatureResponder
    }
}

impl TemperatureSensorService {
    responder_service_process_request_functions!(TemperatureCommandApp);

    fn run_request(&mut self, matched: &ArgMatches) -> Result<String> {
        match matched.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m),
            ("device", Some(_m)) => self.process_device_request(_m),
            ("find", None) => Ok(self.endpoint.set_find_mode()?.to_reply_string()),
            ("led", Some(_m)) => self.process_led_request(_m),
            ("protocol-lock", Some(_m)) => self.process_protocol_lock_request(_m),
            ("read", None) => Ok(self.endpoint.get_reading()?.to_reply_string()),
            ("sleep", None) => Ok(self.endpoint.set_sleep()?.to_reply_string()),
            _ => return Err(ErrorKind::CommandParse.into()),
        }
    }

    // Process calibration request commands.
    fn process_calibration_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("status", None) => Ok(self.endpoint.get_calibration_status()?.to_reply_string()),
            ("clear", None) => Ok(self.endpoint.set_calibration_clear()?.to_reply_string()),
            ("set", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_temperature(cal)?.to_reply_string())
            }
            _ => unreachable!(),
        }
    }
}
