//! Client for Temperature sensing.
use errors::*;
use network::{Endpoint, SocketReply, SocketRequest};
use network::temperature::requests::*;

use neuras;


/// Creates a client for network requests to the `PhResponder`.
network_socket!(TemperatureRequester, "Socket that communicates with the pH sensor.");

impl TemperatureRequester {
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

impl TemperatureRequester {
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

    /// Set the calibration temperature for the sensor.
    pub fn set_calibration_temperature(&mut self, t: f64) -> Result<String> {
        let reply = CalibrationTemperature(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl TemperatureRequester {
    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    pub fn set_data_logger_interval(&mut self, n: u32) -> Result<String> {
        let reply = DataloggerPeriod(n).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Disable the data-logger.
    pub fn set_data_logger_off(&mut self) -> Result<String> {
        let reply = DataloggerDisable.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get the current status of the data-logger.
    pub fn get_data_logger_status(&mut self) -> Result<String> {
        let reply = DataloggerInterval.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl TemperatureRequester {
    /// Clear memory readings.
    pub fn set_memory_clear(&mut self) -> Result<String> {
        let reply = MemoryClear.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Recall the next memory reading on the stack.
    pub fn get_memory_recall(&mut self) -> Result<String> {
        let reply = MemoryRecall.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Recall the last memory reading on the stack.
    pub fn get_memory_recall_last(&mut self) -> Result<String> {
        let reply = MemoryRecallLast.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl TemperatureRequester {
    /// Set the current temperature scale to Celsius.
    pub fn set_scale_to_celsius(&mut self) -> Result<String> {
        let reply = ScaleCelsius.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the current temperature scale to Fahrenheit.
    pub fn set_scale_to_fahrenheit(&mut self) -> Result<String> {
        let reply = ScaleFahrenheit.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the current temperature scale to Kelvin.
    pub fn set_scale_to_kelvin(&mut self) -> Result<String> {
        let reply = ScaleKelvin.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get the current temperature scale. Returns a `TemperatureScale` result.
    pub fn get_scale(&mut self) -> Result<String> {
        let reply = ScaleState.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}
