//! Network services for Conductivity sensors.

use cli::conductivity::ConductivityCommandApp;
use config::{SensorConfig, SocketConfig};
use errors::*;
use network::{Endpoint, SocketReply};
use network::conductivity::ConductivityResponder;
use devices::conductivity::ConductivitySensor;
use utilities::{atof, create_and_bind_responder};

use clap::ArgMatches;

use std::thread;
use std::time::Duration;

// Conductivity sensor responder service.
sensor_responder_service! {
    "Conductivity sensor responder service.",
    ConductivitySensorService: {
        ConductivitySensor, ConductivityResponder
    }
}

impl ConductivitySensorService {
    // `fn process_request()`
    responder_service_process_request_functions!(ConductivityCommandApp);

    // Run the request and return the [`String`] output.
    fn run_request(&self, matched: &ArgMatches) -> Result<String> {
        match matched.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m),
            ("compensation", Some(_m)) => self.process_compensation_request(_m),
            ("device", Some(_m)) => self.process_device_request(_m),
            ("find", None) => Ok(self.endpoint.set_find_mode()?.to_reply_string()),
            ("led", Some(_m)) => self.process_led_request(_m),
            ("output", Some(_m)) => self.process_output_request(_m),
            ("protocol-lock", Some(_m)) => self.process_protocol_lock_request(_m),
            ("read", None) => Ok(self.endpoint.get_reading()?.to_reply_string()),
            ("sleep", None) => Ok(self.endpoint.set_sleep()?.to_reply_string()),
            _ => return Err(ErrorKind::CommandParse.into()),
        }
    }

    // Process calibration request commands.
    fn process_calibration_request(&self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("status", None) => Ok(self.endpoint.get_calibration_status()?.to_reply_string()),
            ("clear", None) => Ok(self.endpoint.set_calibration_clear()?.to_reply_string()),
            ("dry", None) => Ok(self.endpoint.set_calibration_dry()?.to_reply_string()),
            ("high", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_high(cal)?.to_reply_string())
            }
            ("low", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_low(cal)?.to_reply_string())
            }
            ("single", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_single(cal)?.to_reply_string())
            }
            _ => unreachable!(),
        }
    }

    // Process compensation request commands.
    fn process_compensation_request(&self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("get", None) => Ok(self.endpoint.get_compensation()?.to_reply_string()),
            ("set", Some(_m)) => {
                let temp = match _m.value_of("TEMP") {
                    Some(t) => atof(t)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_compensation(temp)?.to_reply_string())
            }
            _ => unreachable!(),
        }
    }

    // Process output parameters request commands.
    fn process_output_request(&self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("status", None) => Ok(self.endpoint.get_output_params()?.to_reply_string()),
            ("ec", Some(_m)) => match _m.subcommand() {
                ("off", None) => Ok(
                    self.endpoint
                        .set_output_conductivity_off()?
                        .to_reply_string(),
                ),
                ("on", None) => Ok(
                    self.endpoint
                        .set_output_conductivity_on()?
                        .to_reply_string(),
                ),
                _ => unreachable!(),
            },
            ("salinity", Some(_m)) => match _m.subcommand() {
                ("off", None) => Ok(self.endpoint.set_output_salinity_off()?.to_reply_string()),
                ("on", None) => Ok(self.endpoint.set_output_salinity_on()?.to_reply_string()),
                _ => unreachable!(),
            },
            ("sg", Some(_m)) => match _m.subcommand() {
                ("off", None) => Ok(
                    self.endpoint
                        .set_output_specific_gravity_off()?
                        .to_reply_string(),
                ),
                ("on", None) => Ok(
                    self.endpoint
                        .set_output_specific_gravity_on()?
                        .to_reply_string(),
                ),
                _ => unreachable!(),
            },
            ("tds", Some(_m)) => match _m.subcommand() {
                ("off", None) => Ok(self.endpoint.set_output_tds_off()?.to_reply_string()),
                ("on", None) => Ok(self.endpoint.set_output_tds_on()?.to_reply_string()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
