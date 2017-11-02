//! Server for Conductivity sensing.
use super::replies::*;
use super::replies::ProbeReading as SensorReading;

use errors::*;
use devices::conductivity::ConductivitySensor;
use network::common::{Endpoint, OkReply};

use neuras;


// Define the network socket for directly interacting with the
// Conductivity sensor via I2C.
network_sensor_socket! {
    ConductivityResponder,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}

impl ConductivityResponder {
    sensor_socket_commands!(device_common);
}

impl ConductivityResponder {
    sensor_socket_commands!(calibration_common);

    /// set dry calibration settings.
    pub fn set_calibration_dry(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_calibration_dry()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&mut self, c: f64) -> Result<OkReply> {
        let _response = self.sensor
            .set_calibration_high(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&mut self, c: f64) -> Result<OkReply> {
        let _response = self.sensor
            .set_calibration_low(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// Set the calibration single-point for the sensor.
    pub fn set_calibration_single(&mut self, c: f64) -> Result<OkReply> {
        let _response = self.sensor
            .set_calibration_single(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
}

impl ConductivityResponder {
    sensor_socket_commands!(temperature_compensation);
}

impl ConductivityResponder {
    /// get the output string parameters for sensor readings.
    pub fn get_output_params(&mut self) -> Result<OutputStringStatus> {
        let response = self.sensor
            .get_output_string_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// set the `ec` output string parameter on.
    pub fn set_output_conductivity_on(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_conductivity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `ec` output string parameter on.
    pub fn set_output_conductivity_off(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_conductivity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `salinity` output string parameter on.
    pub fn set_output_salinity_on(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_salinity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `salinity` output string parameter on.
    pub fn set_output_salinity_off(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_salinity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `sg` output string parameter on.
    pub fn set_output_specific_gravity_on(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_specific_gravity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `sg` output string parameter on.
    pub fn set_output_specific_gravity_off(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_specific_gravity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `tds` output string parameter on.
    pub fn set_output_tds_on(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_tds_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set the `tds` output string parameter on.
    pub fn set_output_tds_off(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_output_tds_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
}

impl ConductivityResponder {
    /// set the probe type to `1.0`
    pub fn set_probe_type_one(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_probe_type_one()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
    /// set the probe type to `0.1`
    pub fn set_probe_type_point_one(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_probe_type_point_one()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
    /// set the probe type to `10`
    pub fn set_probe_type_ten(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_probe_type_ten()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
    /// set the probe type to `10`
    pub fn get_probe_type_status(&mut self) -> Result<ProbeType> {
        let response = self.sensor
            .get_probe_type_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }
}
