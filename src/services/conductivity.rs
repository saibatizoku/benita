//! Network services for Conductivity sensors.
///
/// Conductivity sensor server.
use cli::conductivity::ConductivityCommandApp;
use errors::*;
use network::conductivity::ConductivitySensorServer;
use sensors::conductivity::ConductivitySensor;
use services::atof;

use std::thread;
use std::time::Duration;

use neuras;
use neuras::utils::bind_socket;

pub struct ConductivitySensorService {
    pub server: ConductivitySensorServer,
}

impl ConductivitySensorService {
    /// Create a new Conductivity Sensor Service.
    pub fn new(
        rep_url: &str,
        i2c_path: &str,
        i2c_address: u16,
    ) -> Result<ConductivitySensorService> {
        // We initialize our I2C device connection.
        let sensor = ConductivitySensor::new(&i2c_path, i2c_address)
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
        let server = ConductivitySensorServer::new(responder, sensor)?;

        Ok(ConductivitySensorService { server })
    }
    /// Parse and execute incoming requests.
    fn process_request(&mut self) -> Result<String> {
        let request = self.server.recv()?;

        let cli = ConductivityCommandApp::new();
        let split: Vec<&str> = request.as_str().split(" ").collect();
        let matches = cli.get_matches_from_safe(split.as_slice())
            .chain_err(|| ErrorKind::CommandParse)?;
        let response = match matches.subcommand() {
            ("calibration", Some(_m)) => match _m.subcommand() {
                ("status", None) => self.server.get_calibration_status()?,
                ("clear", None) => self.server.set_calibration_clear()?,
                ("dry", None) => self.server.set_calibration_dry()?,
                ("high", Some(_m)) => {
                    let cal = match _m.value_of("CAL") {
                        Some(_cal) => atof(_cal)?,
                        _ => unreachable!(),
                    };
                    self.server.set_calibration_high(cal)?
                }
                ("low", Some(_m)) => {
                    let cal = match _m.value_of("CAL") {
                        Some(_cal) => atof(_cal)?,
                        _ => unreachable!(),
                    };
                    self.server.set_calibration_low(cal)?
                }
                ("single", Some(_m)) => {
                    let cal = match _m.value_of("CAL") {
                        Some(_cal) => atof(_cal)?,
                        _ => unreachable!(),
                    };
                    self.server.set_calibration_single(cal)?
                }
                _ => unreachable!(),
            },
            ("device", Some(_m)) => match _m.subcommand() {
                ("info", None) => self.server.get_device_info()?,
                ("status", None) => self.server.get_device_status()?,
                _ => unreachable!(),
            },
            ("compensation", Some(_m)) => match _m.subcommand() {
                ("get", None) => self.server.get_compensation()?,
                ("set", Some(_m)) => {
                    let temp = match _m.value_of("TEMP") {
                        Some(t) => atof(t)?,
                        _ => unreachable!(),
                    };
                    self.server.set_compensation(temp)?
                }
                _ => unreachable!(),
            },
            ("output", Some(_m)) => match _m.subcommand() {
                ("status", None) => self.server.get_output_params()?,
                ("ec", Some(_m)) => match _m.subcommand() {
                    ("off", None) => self.server.set_output_conductivity_off()?,
                    ("on", None) => self.server.set_output_conductivity_on()?,
                    _ => unreachable!(),
                },
                ("salinity", Some(_m)) => match _m.subcommand() {
                    ("off", None) => self.server.set_output_salinity_off()?,
                    ("on", None) => self.server.set_output_salinity_on()?,
                    _ => unreachable!(),
                },
                ("sg", Some(_m)) => match _m.subcommand() {
                    ("off", None) => self.server.set_output_specific_gravity_off()?,
                    ("on", None) => self.server.set_output_specific_gravity_on()?,
                    _ => unreachable!(),
                },
                ("tds", Some(_m)) => match _m.subcommand() {
                    ("off", None) => self.server.set_output_tds_off()?,
                    ("on", None) => self.server.set_output_tds_on()?,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            ("read", None) => self.server.get_reading()?,
            ("sleep", None) => self.server.set_sleep()?,
            _ => return Err(ErrorKind::CommandParse.into()),
        };

        Ok(response)
    }

    /// Listen for incoming command requests.
    pub fn listen(&mut self) -> Result<()> {
        loop {
            {
                // Parse and process the command.
                let command_response: String = match self.process_request() {
                    Ok(response) => response,
                    _ => "request error".to_string(),
                };
                // Send response to the client.
                let _respond = self.server.send(command_response.as_bytes())?;
            }

            // No work left, so we sleep.
            thread::sleep(Duration::from_millis(1));
        }
    }
}
