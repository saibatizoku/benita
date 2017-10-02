// Client for Conductivity sensing.
use errors::*;

// Needed by the network_socket! macro.
use neuras;

// Define the network client socket for sending requests to the
// `ConductivitySensorServer`.
network_socket! {
    ConductivityClient,
    "Socket that makes requests to the Conductivity sensor server."
}

impl ConductivityClient {
    /// get the output string parameters for sensor readings.
    pub fn get_output_params(&self) -> Result<String> {
        let _read = self.send("get_params".as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = self.recv()
            .chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    /// send the compensation temperature for sensor readings.
    pub fn send_compensate(&self, t: f64) -> Result<String> {
        let calibrate = format!("calibrate {:.*}", 3, t);
        let _read = self.send(calibrate.as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = self.recv()
            .chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    /// get the output string with sensor readings.
    pub fn send_read(&self) -> Result<String> {
        let _read = self.send("read".as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = self.recv()
            .chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }

    /// set the sensor to sleep (low-power) mode.
    pub fn send_sleep(&self) -> Result<String> {
        let _read = self.send("sleep".as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = self.recv()
            .chain_err(|| ErrorKind::CommandResponse)?;
        Ok(response)
    }
}
