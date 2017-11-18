//! API for Conductivity sensor functionality.
use network::conductivity::replies::*;

/// API for Conductivity commands and replies
pub trait ConductivityAPI {
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
    fn get_reading(&self) -> ::std::result::Result<ProbeReading, Self::Error>;
    /// set the sensor to sleep (low-power) mode.
    fn set_sleep(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Set the compensation temperature.
    fn set_compensation(&self, value: f64) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
    /// Get the current compensated temperature value.
    fn get_compensation(&self) -> ::std::result::Result<CompensationValue, Self::Error>;
    /// Clear the sensor's calibration settings.
    fn set_calibration_clear(&self) -> ::std::result::Result<Self::DefaultReply, Self::Error>;
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
