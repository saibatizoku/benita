//! Server for Conductivity sensing.
use super::replies::*;
use super::replies::ProbeReading as SensorReading;
use super::requests::*;

use api::conductivity::ConductivityAPI;
use errors::*;
use devices::conductivity::ConductivitySensor;
use network::common::{Endpoint, ReplyStatus, Responder, SocketRequest};

use neuras;


// Define the network socket for directly interacting with the
// Conductivity sensor via I2C.
network_sensor_socket! {
    ConductivityResponder,
    ConductivitySensor,
    "Socket that responds to Conductivity sensor commands."
}

impl ConductivityAPI for ConductivityResponder {
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

// Return 'err' string, and log it
fn return_error(e: Error) -> String {
    error!("conductivity sensor error: {}", e);
    format!("{:?}", ReplyStatus::Err)
}

impl Responder for ConductivityResponder {
    type Response = String;

    // Match and evaluate commands
    fn evaluate(&self, req: &str) -> Result<String> {
        match req {
            a if CalibrationDry::from_request_str(a).is_ok() => {
                let _req = CalibrationDry::from_request_str(a)?;
                let reply = match self.set_calibration_dry() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if CalibrationHigh::from_request_str(a).is_ok() => {
                let _req = CalibrationHigh::from_request_str(a)?;
                let reply = match self.set_calibration_high(_req.0) {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if CalibrationLow::from_request_str(a).is_ok() => {
                let _req = CalibrationLow::from_request_str(a)?;
                let reply = match self.set_calibration_low(_req.0) {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if CalibrationOnePoint::from_request_str(a).is_ok() => {
                let _req = CalibrationOnePoint::from_request_str(a)?;
                let reply = match self.set_calibration_single(_req.0) {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if CalibrationState::from_request_str(a).is_ok() => {
                let _req = CalibrationState::from_request_str(a)?;
                let reply = match self.get_calibration_status() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if CompensationSet::from_request_str(a).is_ok() => {
                let _req = CompensationSet::from_request_str(a)?;
                let reply = match self.set_compensation(_req.0) {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if CompensationGet::from_request_str(a).is_ok() => {
                let _req = CompensationGet::from_request_str(a)?;
                let reply = match self.get_compensation() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if DeviceAddress::from_request_str(a).is_ok() => {
                let _req = DeviceAddress::from_request_str(a)?;
                let reply = match self.set_device_address(_req.0) {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if DeviceInformation::from_request_str(a).is_ok() => {
                let _req = DeviceInformation::from_request_str(a)?;
                let reply = match self.get_device_info() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Factory::from_request_str(a).is_ok() => {
                let _req = Factory::from_request_str(a)?;
                let reply = match self.set_factory_reset() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Find::from_request_str(a).is_ok() => {
                let _req = Find::from_request_str(a)?;
                let reply = match self.set_find_mode() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Export::from_request_str(a).is_ok() => {
                let _req = Export::from_request_str(a)?;
                let reply = match self.get_export_line() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if ExportInfo::from_request_str(a).is_ok() => {
                let _req = ExportInfo::from_request_str(a)?;
                let reply = match self.get_export_info() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Import::from_request_str(a).is_ok() => {
                let _req = Import::from_request_str(a)?;
                let reply = match self.set_import_line(&_req.0) {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if LedOff::from_request_str(a).is_ok() => {
                let _req = LedOff::from_request_str(a)?;
                let reply = match self.set_led_off() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if LedOn::from_request_str(a).is_ok() => {
                let _req = LedOn::from_request_str(a)?;
                let reply = match self.set_led_on() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if LedState::from_request_str(a).is_ok() => {
                let _req = LedState::from_request_str(a)?;
                let reply = match self.get_led_status() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if OutputState::from_request_str(a).is_ok() => {
                let _req = OutputState::from_request_str(a)?;
                let reply = match self.get_output_params() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Reading::from_request_str(a).is_ok() => {
                let _req = Reading::from_request_str(a)?;
                let reply = match self.get_reading() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Sleep::from_request_str(a).is_ok() => {
                let _req = Sleep::from_request_str(a)?;
                let reply = match self.set_sleep() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            a if Status::from_request_str(a).is_ok() => {
                let _req = Status::from_request_str(a)?;
                let reply = match self.get_device_status() {
                    Ok(rep) => format!("{}", rep),
                    Err(e) => return_error(e),
                };
                Ok(reply)
            }
            _ => {
                error!("bad sensor command");
                Ok(format!("{:?}", ReplyStatus::Err))
            }
        }
    }
}
