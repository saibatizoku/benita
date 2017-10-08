//! Server for Conductivity sensing.
use errors::*;
use network::SocketCommand;
use sensors::conductivity::ConductivitySensor;

use neuras;


// Define the network server socket for directly interacting with the
// Conductivity sensor via I2C.
network_socket! {
    ConductivitySensorServer,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}

impl ConductivitySensorServer {
    /// get the compensation temperature for sensor readings.
    pub fn get_compensation(&mut self) -> Result<String> {
        let response = self.sensor
            .get_compensated_temperature_value()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("compensation value: {}", response.0))
    }

    /// set the compensation temperature for sensor readings.
    pub fn set_compensation(&mut self, t: f64) -> Result<String> {
        let _response = self.sensor
            .set_compensation_temperature(t)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("compensation set: {}", t))
    }
}

impl ConductivitySensorServer {
    /// get the output string parameters for sensor readings.
    pub fn get_output_params(&mut self) -> Result<String> {
        let response = self.sensor
            .get_output_string_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response.to_string())
    }

    /// set the `ec` output string parameter on.
    pub fn set_output_conductivity_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_conductivity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output ec: on".to_string())
    }

    /// set the `ec` output string parameter on.
    pub fn set_output_conductivity_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_conductivity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output ec: off".to_string())
    }

    /// set the `salinity` output string parameter on.
    pub fn set_output_salinity_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_salinity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output salinity: on".to_string())
    }

    /// set the `salinity` output string parameter on.
    pub fn set_output_salinity_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_salinity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output salinity: off".to_string())
    }

    /// set the `sg` output string parameter on.
    pub fn set_output_specific_gravity_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_specific_gravity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output sg: on".to_string())
    }

    /// set the `sg` output string parameter on.
    pub fn set_output_specific_gravity_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_specific_gravity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output sg: off".to_string())
    }

    /// set the `tds` output string parameter on.
    pub fn set_output_tds_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_tds_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output tds: on".to_string())
    }

    /// set the `tds` output string parameter on.
    pub fn set_output_tds_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_tds_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output tds: off".to_string())
    }
}

impl ConductivitySensorServer {
    /// get the output string with sensor readings.
    pub fn get_reading(&mut self) -> Result<String> {
        let response = self.sensor
            .get_reading()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("{:?}", response))
    }
}

impl ConductivitySensorServer {
    /// set the sensor to sleep (low-power) mode.
    pub fn set_sleep(&mut self) -> Result<String> {
        let _sleep = self.sensor
            .set_sleep()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("sleeping".to_string())
    }
}
