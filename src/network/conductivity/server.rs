//! Server for Conductivity sensing.
use errors::*;
use devices::conductivity::ConductivitySensor;

use neuras;


// Define the network server socket for directly interacting with the
// Conductivity sensor via I2C.
device_socket! {
    ConductivitySensorServer,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}

impl ConductivitySensorServer {
    sensor_socket_commands!(device_common);
}

impl ConductivitySensorServer {
    sensor_socket_commands!(calibration_common);

    /// set dry calibration settings.
    pub fn set_calibration_dry(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_calibration_dry()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("calibration-set dry".to_string())
    }

    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&mut self, c: f64) -> Result<String> {
        let _response = self.sensor
            .set_calibration_high(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("calibration-high {}", c))
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&mut self, c: f64) -> Result<String> {
        let _response = self.sensor
            .set_calibration_low(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("calibration-low {}", c))
    }

    /// Set the calibration single-point for the sensor.
    pub fn set_calibration_single(&mut self, c: f64) -> Result<String> {
        let _response = self.sensor
            .set_calibration_single(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("calibration-single-point {}", c))
    }
}

impl ConductivitySensorServer {
    sensor_socket_commands!(temperature_compensation);
}

impl ConductivitySensorServer {
    /// get the output string parameters for sensor readings.
    pub fn get_output_params(&mut self) -> Result<String> {
        let response = self.sensor
            .get_output_string_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("output-params {}", response))
    }

    /// set the `ec` output string parameter on.
    pub fn set_output_conductivity_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_conductivity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-ec on".to_string())
    }

    /// set the `ec` output string parameter on.
    pub fn set_output_conductivity_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_conductivity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-ec off".to_string())
    }

    /// set the `salinity` output string parameter on.
    pub fn set_output_salinity_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_salinity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-salinity on".to_string())
    }

    /// set the `salinity` output string parameter on.
    pub fn set_output_salinity_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_salinity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-salinity off".to_string())
    }

    /// set the `sg` output string parameter on.
    pub fn set_output_specific_gravity_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_specific_gravity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-sg on".to_string())
    }

    /// set the `sg` output string parameter on.
    pub fn set_output_specific_gravity_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_specific_gravity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-sg off".to_string())
    }

    /// set the `tds` output string parameter on.
    pub fn set_output_tds_on(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_tds_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-tds on".to_string())
    }

    /// set the `tds` output string parameter on.
    pub fn set_output_tds_off(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_output_tds_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("output-tds off".to_string())
    }
}

impl ConductivitySensorServer {
    /// set the probe type to `1.0`
    pub fn set_probe_type_one(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_probe_type_one()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("probe-type 1.0".to_string())
    }
    /// set the probe type to `0.1`
    pub fn set_probe_type_point_one(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_probe_type_point_one()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("probe-type 0.1".to_string())
    }
    /// set the probe type to `10`
    pub fn set_probe_type_ten(&mut self) -> Result<String> {
        let _response = self.sensor
            .set_probe_type_ten()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok("probe-type 10".to_string())
    }
    /// set the probe type to `10`
    pub fn get_probe_type_status(&mut self) -> Result<String> {
        let response = self.sensor
            .get_probe_type_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("probe-type {}", response))
    }
}
