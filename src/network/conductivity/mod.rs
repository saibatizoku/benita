//! Networked services for Conductivity sensing.
use errors::*;

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

// Define the network client socket for sending requests to the
// `ConductivitySensorServer`.
network_socket! {
    ConductivityClient,
    "Socket that makes requests to the Conductivity sensor server."
}

impl ConductivityClient {
    pub fn get_output_params(&mut self) -> Result<String> {
        let _read = self.send("get_params".as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(resp) => Ok(resp.to_string()),
            _ => Err(ErrorKind::SocketReceive.into()),
        }
    }

    pub fn send_compensate(&mut self, t: f64) -> Result<String> {
        let calibrate = format!("calibrate {:.*}", 3, t);
        let _read = self.send(calibrate.as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(resp) => Ok(resp.to_string()),
            _ => Err(ErrorKind::SocketReceive.into()),
        }
    }

    pub fn send_read(&mut self) -> Result<String> {
        let _read = self.send("read".as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(reading) => Ok(reading.to_string()),
            _ => Err(ErrorKind::SocketReceive.into()),
        }
    }

    pub fn send_sleep(&mut self) -> Result<String> {
        let _read = self.send("read".as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(reading) => Ok(reading.to_string()),
            _ => Err(ErrorKind::SocketReceive.into()),
        }
    }
}

// Define the network server socket for directly interacting with the
// Conductivity sensor via I2C.
network_socket! {
    ConductivitySensorServer,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}
