//! Client for Temperature sensing.
pub mod errors {
    error_chain!{}
}

use super::replies::*;
use super::requests::*;
use super::super::TemperatureAPI;

use common_ezo::EzoChipAPI;
use errors::*;
use network::{Endpoint, ReplyStatus, SocketRequest};

use neuras;


// Creates a client for network requests to the `PhResponder`.
network_socket!(
    TemperatureRequester,
    "Socket that communicates with the pH sensor."
);

impl EzoChipAPI for TemperatureRequester {
    type SensorError = Error;
    type SensorReply = ReplyStatus;

    /// Clear the sensor's calibration settings.
    fn set_calibration_clear(&self) -> Result<ReplyStatus> {
        let reply = CalibrationClear.send(self)?;
        Ok(reply)
    }

    /// get the export information from the sensor.
    fn get_export_info(&self) -> Result<ExportedInfo> {
        let reply = ExportInfo.send(self)?;
        Ok(reply)
    }

    /// export a calibration line from the sensor.
    fn get_export_line(&self) -> Result<Exported> {
        let reply = Export.send(self)?;
        Ok(reply)
    }

    /// import a calibration line to the sensor.
    fn set_import_line(&self, import: &str) -> Result<ReplyStatus> {
        let reply = Import(import.to_string()).send(self)?;
        Ok(reply)
    }

    /// get the sensor information.
    fn get_device_info(&self) -> Result<DeviceInfo> {
        let reply = DeviceInformation.send(self)?;
        Ok(reply)
    }

    /// get the sensor status.
    fn get_device_status(&self) -> Result<DeviceStatus> {
        let reply = Status.send(self)?;
        Ok(reply)
    }

    /// reset the sensor device.
    fn set_factory_reset(&self) -> Result<ReplyStatus> {
        let reply = Factory.send(self)?;
        Ok(reply)
    }

    /// set the sensor to find mode.
    fn set_find_mode(&self) -> Result<ReplyStatus> {
        let reply = Find.send(self)?;
        Ok(reply)
    }

    /// change the sensor's I2C address.
    fn set_device_address(&self, address: u16) -> Result<ReplyStatus> {
        let reply = DeviceAddress(address).send(self)?;
        Ok(reply)
    }

    /// set the LED off.
    fn set_led_off(&self) -> Result<ReplyStatus> {
        let reply = LedOff.send(self)?;
        Ok(reply)
    }

    /// set the LED on.
    fn set_led_on(&self) -> Result<ReplyStatus> {
        let reply = LedOn.send(self)?;
        Ok(reply)
    }

    /// get the current LED status.
    fn get_led_status(&self) -> Result<LedStatus> {
        let reply = LedState.send(self)?;
        Ok(reply)
    }

    /// set the protocol lock off.
    fn set_protocol_lock_off(&self) -> Result<ReplyStatus> {
        let reply = ProtocolLockDisable.send(self)?;
        Ok(reply)
    }

    /// set the protocol lock on.
    fn set_protocol_lock_on(&self) -> Result<ReplyStatus> {
        let reply = ProtocolLockEnable.send(self)?;
        Ok(reply)
    }

    /// get the current protocol lock status.
    fn get_protocol_lock_status(&self) -> Result<ProtocolLockStatus> {
        let reply = ProtocolLockState.send(self)?;
        Ok(reply)
    }
    /// set the sensor to sleep (low-power) mode.
    fn set_sleep(&self) -> Result<ReplyStatus> {
        let reply = Sleep.send(self)?;
        Ok(reply)
    }
}

impl TemperatureAPI for TemperatureRequester {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> Result<CalibrationStatus> {
        let reply = CalibrationState.send(self)?;
        Ok(reply)
    }

    /// Set the calibration temperature for the sensor.
    fn set_calibration_temperature(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationTemperature(t).send(self)?;
        Ok(reply)
    }

    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    fn set_data_logger_interval(&self, n: u32) -> Result<ReplyStatus> {
        let reply = DataloggerPeriod(n).send(self)?;
        Ok(reply)
    }

    /// Disable the data-logger.
    fn set_data_logger_off(&self) -> Result<ReplyStatus> {
        let reply = DataloggerDisable.send(self)?;
        Ok(reply)
    }

    /// Get the current status of the data-logger.
    fn get_data_logger_status(&self) -> Result<DataLoggerStorageIntervalSeconds> {
        let reply = DataloggerInterval.send(self)?;
        Ok(reply)
    }

    /// Clear memory readings.
    fn set_memory_clear(&self) -> Result<ReplyStatus> {
        let reply = MemoryClear.send(self)?;
        Ok(reply)
    }

    /// Recall the next memory reading on the stack.
    fn get_memory_recall(&self) -> Result<MemoryReading> {
        let reply = MemoryRecall.send(self)?;
        Ok(reply)
    }

    /// Recall the last memory reading on the stack.
    fn get_memory_recall_last(&self) -> Result<MemoryReading> {
        let reply = MemoryRecallLast.send(self)?;
        Ok(reply)
    }

    /// get the output string with sensor readings.
    fn get_reading(&self) -> Result<SensorReading> {
        let reply = Reading.send(self)?;
        Ok(reply)
    }

    /// Set the current temperature scale to Celsius.
    fn set_scale_to_celsius(&self) -> Result<ReplyStatus> {
        let reply = ScaleCelsius.send(self)?;
        Ok(reply)
    }

    /// Set the current temperature scale to Fahrenheit.
    fn set_scale_to_fahrenheit(&self) -> Result<ReplyStatus> {
        let reply = ScaleFahrenheit.send(self)?;
        Ok(reply)
    }

    /// Set the current temperature scale to Kelvin.
    fn set_scale_to_kelvin(&self) -> Result<ReplyStatus> {
        let reply = ScaleKelvin.send(self)?;
        Ok(reply)
    }

    /// Get the current temperature scale. Returns a `TemperatureScale` result.
    fn get_scale(&self) -> Result<TemperatureScale> {
        let reply = ScaleState.send(self)?;
        Ok(reply)
    }
}
