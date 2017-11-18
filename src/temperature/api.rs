//! API for Temperature sensor funcionality.
use network::temperature::replies::*;

/// API for pH commands and replies
pub trait TemperatureAPI {
    type Error;
    type DefaultReply;

    /// get the export information from the sensor.
    fn get_export_info(&self) -> ::std::result::Result<ExportedInfo, Self::Error>;
    /// export a calibration line from the sensor.
    fn get_export_line(&self) -> ::std::result::Result<Exported, Self::Error>;
    /// import a calibration line to the sensor.
    fn set_import_line(&self, import: &str) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// get the sensor information.
    fn get_device_info(&self) -> ::std::result::Result<DeviceInfo, Self::Error>;
    /// get the sensor status.
    fn get_device_status(&self) -> ::std::result::Result<DeviceStatus, Self::Error>;
    /// reset the sensor device.
    fn set_factory_reset(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// set the sensor to find mode.
    fn set_find_mode(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// change the sensor's I2C address.
    fn set_device_address(&self, address: u16) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// set the LED off.
    fn set_led_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// set the LED on.
    fn set_led_on(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// get the current LED status.
    fn get_led_status(&self) -> ::std::result::Result<LedStatus, Self::Error>;
    /// set the protocol lock off.
    fn set_protocol_lock_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// set the protocol lock on.
    fn set_protocol_lock_on(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// get the current protocol lock status.
    fn get_protocol_lock_status(&self) -> ::std::result::Result<ProtocolLockStatus, Self::Error>;
    /// get the output string with sensor readings.
    fn get_reading(&self) -> ::std::result::Result<SensorReading, Self::Error>;
    /// set the sensor to sleep (low-power) mode.
    fn set_sleep(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Clear the sensor's calibration settings.
    fn set_calibration_clear(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> ::std::result::Result<CalibrationStatus, Self::Error>;
    /// Set the calibration temperature for the sensor.
    fn set_calibration_temperature(&self, t: f64) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    fn set_data_logger_interval(&self, n: u32) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Disable the data-logger.
    fn set_data_logger_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current status of the data-logger.
    fn get_data_logger_status(&self) -> ::std::result::Result<DataLoggerStorageIntervalSeconds, Self::Error>;
    /// Clear memory readings.
    fn set_memory_clear(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Recall the next memory reading on the stack.
    fn get_memory_recall(&self) -> ::std::result::Result<MemoryReading, Self::Error>;
    /// Recall the last memory reading on the stack.
    fn get_memory_recall_last(&self) -> ::std::result::Result<MemoryReading, Self::Error>;
    /// Set the current temperature scale to Celsius.
    fn set_scale_to_celsius(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the current temperature scale to Fahrenheit.
    fn set_scale_to_fahrenheit(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the current temperature scale to Kelvin.
    fn set_scale_to_kelvin(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current temperature scale. Returns a `TemperatureScale` result.
    fn get_scale(&self) -> ::std::result::Result<TemperatureScale, Self::Error>;
}
