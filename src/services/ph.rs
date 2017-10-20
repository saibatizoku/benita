//! Network services for pH sensors.

use cli::ph::PhCommandApp;
use config::{SensorConfig, SocketConfig};
use errors::*;
use network::ph::PhSensorServer;
use devices::ph::PhSensor;
use utilities::{atof, create_and_bind_responder};

use clap::ArgMatches;

use std::thread;
use std::time::Duration;


// pH sensor server.
responder_service! {
    "pH sensor server.",
    PhSensorService: {
        PhSensor, PhSensorServer
    }
}

impl PhSensorService {
    responder_service_process_request_functions!(PhCommandApp);

    fn run_request(&mut self, matched: &ArgMatches) -> Result<String> {
        match matched.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m),
            ("compensation", Some(_m)) => self.process_compensation_request(_m),
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
            ("high", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_high(cal)
            }
            ("mid", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_mid(cal)
            }
            ("low", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_low(cal)
            }
            _ => unreachable!(),
        }
    }

    // Process compensation request commands.
    fn process_compensation_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("get", None) => self.server.get_compensation(),
            ("set", Some(_m)) => {
                let temp = match _m.value_of("TEMP") {
                    Some(t) => atof(t)?,
                    _ => unreachable!(),
                };
                self.server.set_compensation(temp)
            }
            _ => unreachable!(),
        }
    }
}
