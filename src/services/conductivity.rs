//! Network services for Conductivity sensors.

use cli::conductivity::ConductivityCommandApp;
use config::{SensorConfig, SocketConfig};
use errors::*;
use network::conductivity::ConductivitySensorServer;
use sensors::conductivity::ConductivitySensor;
use utilities::atof;

use clap::ArgMatches;

use std::thread;
use std::time::Duration;

use neuras;
use neuras::utils::bind_socket;

/// Conductivity sensor server.
pub struct ConductivitySensorService {
    pub server: ConductivitySensorServer,
}

impl ConductivitySensorService {
    /// Create a new Conductivity Sensor Service.
    pub fn new(
        socket: SocketConfig,
        sensor: SensorConfig,
    ) -> Result<ConductivitySensorService> {
        // We initialize our I2C device connection.
        let conductivity_sensor = ConductivitySensor::new(sensor.path, sensor.address)
            .chain_err(|| "Could not open I2C device")?;

        // We start our ZMQ context.
        let context = neuras::utils::create_context();
        // We configure our socket as REP, for accepting requests
        // and providing REsPonses.
        let responder = neuras::utils::zmq_rep(&context)?;
        // We bind our socket to REP_URL.
        let _bind_socket =
            bind_socket(&responder, rep_url).chain_err(|| "problems binding to socket")?;
        // Setup our sensor server
        let server = ConductivitySensorServer::new(responder, conductivity_sensor)?;

        Ok(ConductivitySensorService { server })
    }

    /// Parse and execute incoming requests.
    pub fn process_request(&mut self) -> Result<String> {
        // Receive the incoming request
        let request = self.server.recv()?;
        let cmd_args: Vec<&str> = request.as_str().split(" ").collect();

        // Start the command-line interpreter
        let cli = ConductivityCommandApp::new();
        let matches = cli.get_matches_from_safe(cmd_args.as_slice())
            .chain_err(|| ErrorKind::CommandParse)?;

        // Match the request subcommands to the service API.
        let response = match matches.subcommand() {
            ("calibration", Some(_m)) => self.process_calibration_request(_m)?,
            ("compensation", Some(_m)) => self.process_compensation_request(_m)?,
            ("device", Some(_m)) => self.process_device_request(_m)?,
            ("find", None) => self.server.set_find_mode()?,
            ("led", Some(_m)) => self.process_led_request(_m)?,
            ("output", Some(_m)) => self.process_output_request(_m)?,
            ("protocol-lock", Some(_m)) => self.process_protocol_lock_request(_m)?,
            ("read", None) => self.server.get_reading()?,
            ("sleep", None) => self.server.set_sleep()?,
            _ => return Err(ErrorKind::CommandParse.into()),
        };

        // Return the response string.
        Ok(response)
    }

    // Process device request commands.
    fn process_device_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("address", Some(_m)) => {
                let val = match _m.value_of("ADDRESS") {
                    Some(_val) => _val.parse::<u16>().chain_err(|| "not a number")?,
                    _ => unreachable!(),
                };
                self.server.set_device_address(val)
            }
            ("info", None) => self.server.get_device_info(),
            ("factory", None) => self.server.set_factory_reset(),
            ("status", None) => self.server.get_device_status(),
            _ => unreachable!(),
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

    // Process LED request commands.
    fn process_led_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("off", None) => self.server.set_led_off(),
            ("on", None) => self.server.set_led_on(),
            ("status", None) => self.server.get_led_status(),
            _ => unreachable!(),
        }
    }

    // Process protocol-lock request commands.
    fn process_protocol_lock_request(&mut self, matches: &ArgMatches) -> Result<String> {
        match matches.subcommand() {
            ("off", None) => self.server.set_protocol_lock_off(),
            ("on", None) => self.server.set_protocol_lock_on(),
            ("status", None) => self.server.get_protocol_lock_status(),
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

    /// Listen for incoming command requests.
    pub fn listen(&mut self) -> Result<()> {
        loop {
            {
                // Parse and process the command.
                let command_response: String = match self.process_request() {
                    Ok(response) => response,
                    _ => "error".to_string(),
                };
                // Send response to the client.
                let _respond = self.server.send(command_response.as_bytes())?;
            }

            // No work left, so we sleep.
            thread::sleep(Duration::from_millis(1));
        }
    }
}
