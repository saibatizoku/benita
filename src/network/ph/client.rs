//! Client for pH sensing.
use errors::*;
use network::Endpoint;

use neuras;


/// Creates a client for network requests of the ph sensor.
network_socket!(PhClient, "Socket that communicates with the pH sensor.");

impl PhClient {
    pub fn get_output_params(&self) -> Result<String> {
        let _send = self.send("get_params".as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    pub fn send_compensate(&self, t: f64) -> Result<String> {
        let compensate = format!("calibrate {:.*}", 3, t);
        let _send = self.send(compensate.as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    pub fn send_read(&self) -> Result<String> {
        let _send = self.send("read".as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    pub fn send_sleep(&self) -> Result<String> {
        let _send = self.send("sleep".as_bytes())?;
        let response = self.recv().chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }
}
