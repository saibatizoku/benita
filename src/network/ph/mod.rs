//! Networked services for pH sensing.
use errors::*;

use neuras;


/// Creates a client for network requests of the ph sensor.
network_socket!(PhClient, "Socket that communicates with the pH sensor.");

impl PhClient {
    pub fn get_output_params(&self) -> Result<String> {
        let _read = self.send("get_params".as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    pub fn send_compensate(&self, t: f64) -> Result<String> {
        let calibrate = format!("calibrate {:.*}", 3, t);
        let _read = self.send(calibrate.as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    pub fn send_read(&self) -> Result<String> {
        let _read = self.send("read".as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    pub fn send_sleep(&self) -> Result<String> {
        let _read = self.send("read".as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }
}
