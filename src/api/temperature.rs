//! API for Temperature sensor funcionality.
use errors::*;

use network::temperature::replies::*;

/// API for pH commands and replies
pub trait TemperatureAPI {
    type DefaultReply;

    /// get the export information from the sensor.
    fn get_export_info(&self) -> Result<ExportedInfo>;
    /// export a calibration line from the sensor.
    fn get_export_line(&self) -> Result<Exported>;
    /// import a calibration line to the sensor.
    fn set_import_line(&self, import: &str) -> Result<Self::DefaultReply>;
    /// get the sensor information.
    fn get_device_info(&self) -> Result<DeviceInfo>;
    /// get the sensor status.
    fn get_device_status(&self) -> Result<DeviceStatus>;
    /// reset the sensor device.
    fn set_factory_reset(&self) -> Result<Self::DefaultReply>;
    /// set the sensor to find mode.
    fn set_find_mode(&self) -> Result<Self::DefaultReply>;
    /// change the sensor's I2C address.
    fn set_device_address(&self, address: u16) -> Result<Self::DefaultReply>;
    /// set the LED off.
    fn set_led_off(&self) -> Result<Self::DefaultReply>;
    /// set the LED on.
    fn set_led_on(&self) -> Result<Self::DefaultReply>;
    /// get the current LED status.
    fn get_led_status(&self) -> Result<LedStatus>;
    /// set the protocol lock off.
    fn set_protocol_lock_off(&self) -> Result<Self::DefaultReply>;
    /// set the protocol lock on.
    fn set_protocol_lock_on(&self) -> Result<Self::DefaultReply>;
    /// get the current protocol lock status.
    fn get_protocol_lock_status(&self) -> Result<ProtocolLockStatus>;
    /// get the output string with sensor readings.
    fn get_reading(&self) -> Result<SensorReading>;
    /// set the sensor to sleep (low-power) mode.
    fn set_sleep(&self) -> Result<Self::DefaultReply>;
    /// Clear the sensor's calibration settings.
    fn set_calibration_clear(&self) -> Result<Self::DefaultReply>;
    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> Result<CalibrationStatus>;
    /// Set the calibration temperature for the sensor.
    fn set_calibration_temperature(&self, t: f64) -> Result<Self::DefaultReply>;
    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    fn set_data_logger_interval(&self, n: u32) -> Result<Self::DefaultReply>;
    /// Disable the data-logger.
    fn set_data_logger_off(&self) -> Result<Self::DefaultReply>;
    /// Get the current status of the data-logger.
    fn get_data_logger_status(&self) -> Result<DataLoggerStorageIntervalSeconds>;
    /// Clear memory readings.
    fn set_memory_clear(&self) -> Result<Self::DefaultReply>;
    /// Recall the next memory reading on the stack.
    fn get_memory_recall(&self) -> Result<MemoryReading>;
    /// Recall the last memory reading on the stack.
    fn get_memory_recall_last(&self) -> Result<MemoryReading>;
    /// Set the current temperature scale to Celsius.
    fn set_scale_to_celsius(&self) -> Result<Self::DefaultReply>;
    /// Set the current temperature scale to Fahrenheit.
    fn set_scale_to_fahrenheit(&self) -> Result<Self::DefaultReply>;
    /// Set the current temperature scale to Kelvin.
    fn set_scale_to_kelvin(&self) -> Result<Self::DefaultReply>;
    /// Get the current temperature scale. Returns a `TemperatureScale` result.
    fn get_scale(&self) -> Result<TemperatureScale>;
}
