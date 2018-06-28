//! API for pH sensor functionality.
use super::response::*;
use common_ezo::EzoChipAPI;

/// API for pH commands and replies
pub trait PhAPI: EzoChipAPI {
    type Error;
    type DefaultReply;

    /// get the export information from the sensor.
    /// get the output string with sensor readings.
    fn get_reading(&self) -> ::std::result::Result<SensorReading, Self::Error>;
    /// Set the compensation temperature.
    fn set_compensation(
        &self,
        value: f64,
    ) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current compensated temperature value.
    fn get_compensation(&self) -> ::std::result::Result<CompensationValue, Self::Error>;
    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> ::std::result::Result<CalibrationStatus, Self::Error>;
    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(
        &self,
        t: f64,
    ) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64)
        -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the value for mid-point calibration.
    fn set_calibration_mid(&self, t: f64)
        -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current slope for the pH sensor.
    fn get_slope(&self) -> ::std::result::Result<ProbeSlope, Self::Error>;
}
