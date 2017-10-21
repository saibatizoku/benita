//! Network services for Temperature sensors.

use cli::temperature::TemperatureCommandApp;
use config::{SensorConfig, SocketConfig};
use errors::*;
use network::Endpoint;
use network::temperature::TemperatureSensorServer;
use devices::temperature::TemperatureSensor;
use utilities::{atof, create_and_bind_responder};

use clap::ArgMatches;

use std::thread;
use std::time::Duration;


// Temperature sensor server.
responder_service! {
    "Temperature sensor server.",
    TemperatureSensorService: {
        TemperatureSensor, TemperatureSensorServer
    }
}

impl TemperatureSensorService {
    responder_service_process_request_functions!(TemperatureCommandApp);

    fn run_request(&mut self, matched: &ArgMatches) -> Result<String> {
        match matched.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m),
            ("device", Some(_m)) => self.process_device_request(_m),
            ("find", None) => self.server.set_find_mode(),
            ("led", Some(_m)) => self.process_led_request(_m),
            ("protocol-lock", Some(_m)) => self.process_protocol_lock_request(_m),
            ("read", None) => self.server.get_reading(),
            ("sleep", None) => self.server.set_sleep(),
            _ => return Err(ErrorKind::CommandParse.into()),
        }
    }

    // Process calibration request commands.
    fn process_calibration_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("status", None) => self.server.get_calibration_status(),
            ("clear", None) => self.server.set_calibration_clear(),
            ("set", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_temperature(cal)
            }
            _ => unreachable!(),
        }
    }
}
