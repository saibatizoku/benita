//! Server for Conductivity sensing.
pub mod errors {
    error_chain! {
    }
}

use super::replies::*;
use super::replies::ProbeReading as SensorReading;

use api::conductivity::ConductivityAPI;
use errors::*;
use devices::conductivity::ConductivitySensor;
use network::common::{Endpoint, ReplyStatus};

use neuras;


// Define the network socket for directly interacting with the
// Conductivity sensor via I2C.
network_sensor_socket! {
    ConductivityResponder,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}

impl ConductivityAPI for ConductivityResponder {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_socket_commands!(device_common);

    sensor_socket_commands!(calibration_common);

    /// set dry calibration settings.
    fn set_calibration_dry(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_dry()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_high(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_low(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration single-point for the sensor.
    fn set_calibration_single(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_single(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    sensor_socket_commands!(temperature_compensation);

    /// get the output string parameters for sensor readings.
    fn get_output_params(&self) -> Result<OutputStringStatus> {
        let response = self.sensor
            .get_output_params()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// set the `ec` output string parameter on.
    fn set_output_conductivity_on(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_conductivity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `ec` output string parameter on.
    fn set_output_conductivity_off(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_conductivity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `salinity` output string parameter on.
    fn set_output_salinity_on(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_salinity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `salinity` output string parameter on.
    fn set_output_salinity_off(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_salinity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `sg` output string parameter on.
    fn set_output_specific_gravity_on(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_specific_gravity_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `sg` output string parameter on.
    fn set_output_specific_gravity_off(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_specific_gravity_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `tds` output string parameter on.
    fn set_output_tds_on(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_tds_on()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the `tds` output string parameter on.
    fn set_output_tds_off(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_output_tds_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the probe type to `1.0`
    fn set_probe_type_one(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_probe_type_one()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }
    /// set the probe type to `0.1`
    fn set_probe_type_point_one(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_probe_type_point_one()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }
    /// set the probe type to `10`
    fn set_probe_type_ten(&self) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_probe_type_ten()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }
    /// set the probe type to `10`
    fn get_probe_type_status(&self) -> Result<ProbeType> {
        let response = self.sensor
            .get_probe_type_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }
}
