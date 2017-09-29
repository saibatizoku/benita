//! Networked services for pH sensing.
use errors::*;

use neuras;


/// Creates a client for network requests of the ph sensor.
network_socket!(PhClient,
               "Socket that communicates with the pH sensor.");

impl PhClient {
    pub fn get_output_params(&mut self) -> Result<String> {
        let _read = self.send("get_params".as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(resp) => Ok(resp.to_string()),
            _ => Err(ErrorKind::CommandResponse.into()),
        }
    }

    pub fn send_compensate(&mut self, t: f64) -> Result<String> {
        let calibrate = format!("calibrate {:.*}", 3, t);
        let _read = self.send(calibrate.as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(resp) => Ok(resp.to_string()),
            _ => Err(ErrorKind::CommandResponse.into()),
        }
    }

    pub fn send_read(&mut self) -> Result<String> {
        let _read = self.send("read".as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(reading) => Ok(reading.to_string()),
            _ => Err(ErrorKind::CommandResponse.into()),
        }
    }

    pub fn send_sleep(&mut self) -> Result<String> {
        let _read = self.send("read".as_bytes())?;
        let _response = self.recv()?;
        match _response.as_str() {
            Some(reading) => Ok(reading.to_string()),
            _ => Err(ErrorKind::CommandResponse.into()),
        }
    }
}
