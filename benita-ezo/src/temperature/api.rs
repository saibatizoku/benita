//! API for Temperature sensor funcionality.
use super::response::*;

use common_ezo::EzoChipAPI;

/// API for pH commands and replies
pub trait TemperatureAPI: EzoChipAPI {
    type Error;
    type DefaultReply;

    /// get the output string with sensor readings.
    fn get_reading(&self) -> ::std::result::Result<SensorReading, Self::Error>;
    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> ::std::result::Result<CalibrationStatus, Self::Error>;
    /// Set the calibration temperature for the sensor.
    fn set_calibration_temperature(
        &self,
        t: f64,
    ) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    fn set_data_logger_interval(
        &self,
        n: u32,
    ) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Disable the data-logger.
    fn set_data_logger_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current status of the data-logger.
    fn get_data_logger_status(
        &self,
    ) -> ::std::result::Result<DataLoggerStorageIntervalSeconds, Self::Error>;
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
