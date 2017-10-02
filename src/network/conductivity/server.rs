//! Server for Conductivity sensing.
pub mod commands {
    use errors::*;
    use super::{SocketCommand, ConductivitySensorServer};

    macro_rules! conductivity_command {
        ( $name:ident , $response:ty ,
          $resp:ident : $runfn:block,
          $doc:tt ) => {
              socket_command! {
                  $name, SocketCommand,
                  ConductivitySensorServer,
                  response : $response,
                  $resp : $runfn,
                  $doc
              }
          };
    }

    conductivity_command! {
        ReadCommand, String,
        responder: {
            let response = responder.sensor.get_reading()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("{:?}", response))
        },
        "Read command"
    }

    conductivity_command! {
        SleepCommand, (),
        responder: {
            let _response = responder.sensor.set_sleep()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(())
        },
        "Network command for sleep mode."
    }
}

use errors::*;
use sensors::conductivity::ConductivitySensor;

use neuras;

/// REP command-set.
///
/// *   `calibrate n` command, where n is a temperature float/int.
/// *   `get_params` command, return the output readings configuration.
/// *   `read` command, returns the output readings.
/// *   `sleep` command, sets the device to sleep/low-power mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum REPCommand {
    Calibrate(f64),
    GetParams,
    Read,
    Sleep,
}

impl REPCommand {
    pub fn parse(cmd_str: &str) -> Result<REPCommand> {
        let cmd = match cmd_str {
            "read" => REPCommand::Read,
            a if cmd_str.starts_with("calibrate ") => {
                let rest = a.get(10..).unwrap();
                let temp = rest.parse().unwrap();
                REPCommand::Calibrate(temp)
            }
            "get_params" => REPCommand::GetParams,
            "sleep" => REPCommand::Sleep,
            _ => return Err(ErrorKind::CommandParse.into()),
        };
        Ok(cmd)
    }
}
// Define the network server socket for directly interacting with the
// Conductivity sensor via I2C.
network_socket! {
    ConductivitySensorServer,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}

impl ConductivitySensorServer {
    pub fn process_request(&mut self, cli_command: REPCommand) -> Result<String> {
        // Parse and process the command.
        let parsed = match cli_command {
            REPCommand::Calibrate(temp) => match self.set_compensation(temp) {
                Ok(_) => format!("temperature-compensation {}", temp),
                Err(e) => format!("error {}", e),
            }
            REPCommand::GetParams => match self.get_output_params() {
                Ok(output_state) => output_state.to_string(),
                Err(e) => format!("error {}", e),
            }
            REPCommand::Read => match self.get_reading() {
                Ok(sensor_output) => format!("{:?}", sensor_output),
                Err(e) => format!("error {}", e),
            }
            REPCommand::Sleep => match self.set_sleep() {
                Ok(_) => "sleeping".to_string(),
                Err(e) => format!("error {}", e),
            }
        };
        Ok(parsed)
    }
}

impl ConductivitySensorServer {
    /// set the compensation temperature for sensor readings.
    pub fn set_compensation(&mut self, t: f64) -> Result<String> {
        let _response = self.sensor.set_compensation_temperature(t)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("temperature-compensated {}", t))
    }

    /// get the output string parameters for sensor readings.
    pub fn get_output_params(&mut self) -> Result<String> {
        let response = self.sensor.get_output_string_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response.to_string())
    }

    /// get the output string with sensor readings.
    pub fn get_reading(&mut self) -> Result<String> {
        let response = self.sensor.get_reading()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("{:?}", response))
    }

    /// set the sensor to sleep (low-power) mode.
    pub fn set_sleep(&mut self) -> Result<String> {
        let _response = self.sensor.set_sleep()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("sleeping".to_string())
    }
}
