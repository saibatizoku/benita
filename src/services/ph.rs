//! Network services for pH sensors.

use cli::ph::PhCommandApp;
use config::{SensorConfig, SocketConfig};
use errors::*;
use network::{Endpoint, SocketReply};
use ph::PhAPI;
use ph::device::PhSensor;
use ph::network::PhResponder;
use utilities::{atof, create_and_bind_responder};

use clap::ArgMatches;

use std::thread;
use std::time::Duration;


// pH sensor responder service.
sensor_responder_service! {
    "pH sensor responder service.",
    PhSensorService: {
        PhSensor, PhResponder
    }
}

impl PhSensorService {
    responder_service_process_request_functions!(PhCommandApp);

    fn run_request(&self, matched: &ArgMatches) -> Result<String> {
        match matched.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m),
            ("compensation", Some(_m)) => self.process_compensation_request(_m),
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
    fn process_calibration_request(&self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("status", None) => Ok(self.endpoint.get_calibration_status()?.to_reply_string()),
            ("clear", None) => Ok(self.endpoint.set_calibration_clear()?.to_reply_string()),
            ("high", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_high(cal)?.to_reply_string())
            }
            ("mid", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_mid(cal)?.to_reply_string())
            }
            ("low", Some(_m)) => {
                let cal = match _m.value_of("CAL") {
                    Some(_cal) => atof(_cal)?,
                    _ => unreachable!(),
                };
                Ok(self.endpoint.set_calibration_low(cal)?.to_reply_string())
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
}
