//! API for Conductivity sensor functionality.
use super::response::*;

use common_ezo::EzoChipAPI;

/// API for Conductivity commands and replies
pub trait ConductivityAPI : EzoChipAPI {
    type Error;
    type DefaultReply;

    /// get the output string with sensor readings.
    fn get_reading(&self) -> ::std::result::Result<SensorReading, Self::Error>;
    /// Set the compensation temperature.
    fn set_compensation(&self, value: f64) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current compensated temperature value.
    fn get_compensation(&self) -> ::std::result::Result<CompensationValue, Self::Error>;
    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> ::std::result::Result<CalibrationStatus, Self::Error>;
    /// Set the value for dry calibration.
    fn set_calibration_dry(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, t: f64) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the calibration single-point for the sensor.
    fn set_calibration_single(&self, t: f64) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Disable conductivity from output.
    fn set_output_conductivity_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Disable salinity from output.
    fn set_output_salinity_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Disable specific gravity from output.
    fn set_output_specific_gravity_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Disable total dissolved solids from output.
    fn set_output_tds_off(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Enable conductivity from output.
    fn set_output_conductivity_on(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Enable salinity from output.
    fn set_output_salinity_on(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Enable specific gravity from output.
    fn set_output_specific_gravity_on(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Enable total dissolved solids from output.
    fn set_output_tds_on(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the output string status.
    fn get_output_params(&self) -> ::std::result::Result<OutputStringStatus, Self::Error>;
    /// Set the probe type to `1.0`.
    fn set_probe_type_one(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the probe type to `0.1`.
    fn set_probe_type_point_one(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the probe type to `10`.
    fn set_probe_type_ten(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get probe type status.
    fn get_probe_type_status(&self) -> ::std::result::Result<ProbeType, Self::Error>;
}
