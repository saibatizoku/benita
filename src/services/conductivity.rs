//! Network services for Conductivity sensors.

use cli::conductivity::ConductivityCommandApp;
use config::{SensorConfig, SocketConfig};
use errors::*;
use network::conductivity::ConductivitySensorServer;
use sensors::conductivity::ConductivitySensor;
use utilities::{atof, create_and_bind_responder};

use clap::ArgMatches;

use std::thread;
use std::time::Duration;

// Conductivity sensor server.
responder_service! {
    "Conductivity sensor server.",
    ConductivitySensorService: {
        ConductivitySensor, ConductivitySensorServer
    }
}

impl ConductivitySensorService {
    // `fn process_request()`
    responder_service_process_request_functions!(ConductivityCommandApp);

    // Run the request and return the [`String`] output.
    fn run_request(&mut self, matched: &ArgMatches) -> Result<String> {
        match matched.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m),
            ("compensation", Some(_m)) => self.process_compensation_request(_m),
            ("device", Some(_m)) => self.process_device_request(_m),
            ("find", None) => self.server.set_find_mode(),
            ("led", Some(_m)) => self.process_led_request(_m),
            ("output", Some(_m)) => self.process_output_request(_m),
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
            ("dry", None) => self.server.set_calibration_dry(),
            ("high", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_high(cal)
            }
            ("low", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_low(cal)
            }
            ("single", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                self.server.set_calibration_single(cal)
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

    // Process output parameters request commands.
    fn process_output_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("status", None) => self.server.get_output_params(),
            ("ec", Some(_m)) => match _m.subcommand() {
                ("off", None) => self.server.set_output_conductivity_off(),
                ("on", None) => self.server.set_output_conductivity_on(),
                _ => unreachable!(),
            },
            ("salinity", Some(_m)) => match _m.subcommand() {
                ("off", None) => self.server.set_output_salinity_off(),
                ("on", None) => self.server.set_output_salinity_on(),
                _ => unreachable!(),
            },
            ("sg", Some(_m)) => match _m.subcommand() {
                ("off", None) => self.server.set_output_specific_gravity_off(),
                ("on", None) => self.server.set_output_specific_gravity_on(),
                _ => unreachable!(),
            },
            ("tds", Some(_m)) => match _m.subcommand() {
                ("off", None) => self.server.set_output_tds_off(),
                ("on", None) => self.server.set_output_tds_on(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
