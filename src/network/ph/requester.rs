//! Client for pH sensing.
use errors::*;
use network::{Endpoint, SocketReply, SocketRequest};
use network::ph::requests::*;

use neuras;


/// Creates a client for network requests to the `PhResponder`.
network_socket!(PhRequester, "Socket that communicates with the pH sensor.");

impl PhRequester {
    /// get the export information from the sensor.
    pub fn get_export_info(&self) -> Result<String> {
        let reply = ExportInfo.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// export a calibration line from the sensor.
    pub fn get_export_line(&self) -> Result<String> {
        let reply = ExportInfo.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// import a calibration line to the sensor.
    pub fn set_import_line(&self, import: &str) -> Result<String> {
        let reply = Import(import.to_string()).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// get the sensor information.
    pub fn get_device_info(&self) -> Result<String> {
        let reply = DeviceInformation.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// get the sensor status.
    pub fn get_device_status(&self) -> Result<String> {
        let reply = Status.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// reset the sensor device.
    pub fn set_factory_reset(&self) -> Result<String> {
        let reply = Factory.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// set the sensor to find mode.
    pub fn set_find_mode(&self) -> Result<String> {
        let reply = Find.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// change the sensor's I2C address.
    pub fn set_device_address(&self, address: u16) -> Result<String> {
        let reply = DeviceAddress(address).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// set the LED off.
    pub fn set_led_off(&self) -> Result<String> {
        let reply = LedOff.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// set the LED on.
    pub fn set_led_on(&self) -> Result<String> {
        let reply = LedOn.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// get the current LED status.
    pub fn get_led_status(&self) -> Result<String> {
        let reply = LedState.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// set the protocol lock off.
    pub fn set_protocol_lock_off(&self) -> Result<String> {
        let reply = ProtocolLockDisable.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// set the protocol lock on.
    pub fn set_protocol_lock_on(&self) -> Result<String> {
        let reply = ProtocolLockEnable.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// get the current protocol lock status.
    pub fn get_protocol_lock_status(&self) -> Result<String> {
        let reply = ProtocolLockState.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// get the output string with sensor readings.
    pub fn get_reading(&self) -> Result<String> {
        let reply = Reading.send_to(self)?;
        Ok(reply.to_reply_string())
    }
    /// set the sensor to sleep (low-power) mode.
    pub fn set_sleep(&self) -> Result<String> {
        let reply = Sleep.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl PhRequester {
    /// Clear the sensor's calibration settings.
    pub fn set_calibration_clear(&mut self) -> Result<String> {
        let reply = CalibrationClear.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get the sensor's current calibration settings.
    pub fn get_calibration_status(&mut self) -> Result<String> {
        let reply = CalibrationState.send_to(self)?;
        Ok(reply.to_reply_string())
    }
    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&mut self, t: f64) -> Result<String> {
        let reply = CalibrationHigh(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&mut self, t: f64) -> Result<String> {
        let reply = CalibrationLow(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the value for mid-point calibration.
    pub fn set_calibration_mid(&mut self, t: f64) -> Result<String> {
        let reply = CalibrationMid(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl PhRequester {
    /// Set the compensation temperature.
    pub fn set_compensation_temperature(&self, value: f64) -> Result<String> {
        let reply = CompensationSet(value).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get the current compensated temperature value.
    pub fn get_compensated_temperature_value(&self) -> Result<String> {
        let reply = CompensationGet.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl PhRequester {
    /// Get the current slope of the pH Sensor.
    pub fn get_slope(&mut self) -> Result<String> {
        let reply = Slope.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}
