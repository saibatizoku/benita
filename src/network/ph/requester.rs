//! Client for pH sensing.
pub mod errors {
    error_chain! {
    }
}

use super::replies::*;
use super::requests::*;

use api::ph::PhAPI;
use errors::*;
use network::common::{Endpoint, ReplyStatus, SocketRequest};

use neuras;


// Creates a client for network requests to the `PhResponder`.
network_socket!(PhRequester, "Socket that communicates with the pH sensor.");

impl PhAPI for PhRequester {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    /// get the export information from the sensor.
    fn get_export_info(&self) -> Result<ExportedInfo> {
        let reply = ExportInfo.send_to(self)?;
        Ok(reply)
    }

    /// export a calibration line from the sensor.
    fn get_export_line(&self) -> Result<Exported> {
        let reply = Export.send_to(self)?;
        Ok(reply)
    }

    /// import a calibration line to the sensor.
    fn set_import_line(&self, import: &str) -> Result<ReplyStatus> {
        let reply = Import(import.to_string()).send_to(self)?;
        Ok(reply)
    }

    /// get the sensor information.
    fn get_device_info(&self) -> Result<DeviceInfo> {
        let reply = DeviceInformation.send_to(self)?;
        Ok(reply)
    }

    /// get the sensor status.
    fn get_device_status(&self) -> Result<DeviceStatus> {
        let reply = Status.send_to(self)?;
        Ok(reply)
    }

    /// reset the sensor device.
    fn set_factory_reset(&self) -> Result<ReplyStatus> {
        let reply = Factory.send_to(self)?;
        Ok(reply)
    }

    /// set the sensor to find mode.
    fn set_find_mode(&self) -> Result<ReplyStatus> {
        let reply = Find.send_to(self)?;
        Ok(reply)
    }

    /// change the sensor's I2C address.
    fn set_device_address(&self, address: u16) -> Result<ReplyStatus> {
        let reply = DeviceAddress(address).send_to(self)?;
        Ok(reply)
    }

    /// set the LED off.
    fn set_led_off(&self) -> Result<ReplyStatus> {
        let reply = LedOff.send_to(self)?;
        Ok(reply)
    }

    /// set the LED on.
    fn set_led_on(&self) -> Result<ReplyStatus> {
        let reply = LedOn.send_to(self)?;
        Ok(reply)
    }

    /// get the current LED status.
    fn get_led_status(&self) -> Result<LedStatus> {
        let reply = LedState.send_to(self)?;
        Ok(reply)
    }

    /// set the protocol lock off.
    fn set_protocol_lock_off(&self) -> Result<ReplyStatus> {
        let reply = ProtocolLockDisable.send_to(self)?;
        Ok(reply)
    }

    /// set the protocol lock on.
    fn set_protocol_lock_on(&self) -> Result<ReplyStatus> {
        let reply = ProtocolLockEnable.send_to(self)?;
        Ok(reply)
    }

    /// get the current protocol lock status.
    fn get_protocol_lock_status(&self) -> Result<ProtocolLockStatus> {
        let reply = ProtocolLockState.send_to(self)?;
        Ok(reply)
    }

    /// get the output string with sensor readings.
    fn get_reading(&self) -> Result<SensorReading> {
        let reply = Reading.send_to(self)?;
        Ok(reply)
    }
    /// set the sensor to sleep (low-power) mode.
    fn set_sleep(&self) -> Result<ReplyStatus> {
        let reply = Sleep.send_to(self)?;
        Ok(reply)
    }

    /// Clear the sensor's calibration settings.
    fn set_calibration_clear(&self) -> Result<ReplyStatus> {
        let reply = CalibrationClear.send_to(self)?;
        Ok(reply)
    }

    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> Result<CalibrationStatus> {
        let reply = CalibrationState.send_to(self)?;
        Ok(reply)
    }
    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationHigh(t).send_to(self)?;
        Ok(reply)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationLow(t).send_to(self)?;
        Ok(reply)
    }

    /// Set the value for mid-point calibration.
    fn set_calibration_mid(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationMid(t).send_to(self)?;
        Ok(reply)
    }

    /// Set the compensation temperature.
    fn set_compensation(&self, value: f64) -> Result<ReplyStatus> {
        let reply = CompensationSet(value).send_to(self)?;
        Ok(reply)
    }

    /// Get the current compensated temperature value.
    fn get_compensation(&self) -> Result<CompensationValue> {
        let reply = CompensationGet.send_to(self)?;
        Ok(reply)
    }

    /// Get the current slope of the pH Sensor.
    fn get_slope(&self) -> Result<ProbeSlope> {
        let reply = Slope.send_to(self)?;
        Ok(reply)
    }
}
