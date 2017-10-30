//! Client for Conductivity sensing.
use errors::*;
use network::{Endpoint, SocketReply, SocketRequest};
use network::conductivity::requests::{CompensationSet, OutputState, Reading, Sleep};

use neuras;

// Define the network client socket for sending requests to a
// `ConductivityClientSocket`.
network_socket! {
    ConductivityClient,
    "Socket that makes requests to the Conductivity sensor socket."
}

impl ConductivityClient {
    /// get the output string parameters for sensor readings.
    pub fn get_output_params(&self) -> Result<String> {
        let reply = OutputState.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// send the compensation temperature for sensor readings.
    pub fn send_compensate(&self, t: f64) -> Result<String> {
        let reply = CompensationSet(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// get the output string with sensor readings.
    pub fn send_read(&self) -> Result<String> {
        let reply = Reading.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// set the sensor to sleep (low-power) mode.
    pub fn send_sleep(&self) -> Result<String> {
        let reply = Sleep.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}
