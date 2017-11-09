//! API for Conductivity sensor functionality.
use errors::*;

use network::conductivity::replies::*;

/// API for Conductivity commands and replies
pub trait ConductivityAPI {
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
    fn get_reading(&self) -> Result<ProbeReading>;
    /// set the sensor to sleep (low-power) mode.
    fn set_sleep(&self) -> Result<Self::DefaultReply>;
    /// Set the compensation temperature.
    fn set_compensation(&self, value: f64) -> Result<Self::DefaultReply>;
    /// Get the current compensated temperature value.
    fn get_compensation(&self) -> Result<CompensationValue>;
    /// Clear the sensor's calibration settings.
    fn set_calibration_clear(&self) -> Result<Self::DefaultReply>;
    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> Result<CalibrationStatus>;
    /// Set the value for dry calibration.
    fn set_calibration_dry(&self) -> Result<Self::DefaultReply>;
    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, t: f64) -> Result<Self::DefaultReply>;
    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64) -> Result<Self::DefaultReply>;
    /// Set the calibration single-point for the sensor.
    fn set_calibration_single(&self, t: f64) -> Result<Self::DefaultReply>;
    /// Disable conductivity from output.
    fn set_output_conductivity_off(&self) -> Result<Self::DefaultReply>;
    /// Disable salinity from output.
    fn set_output_salinity_off(&self) -> Result<Self::DefaultReply>;
    /// Disable specific gravity from output.
    fn set_output_specific_gravity_off(&self) -> Result<Self::DefaultReply>;
    /// Disable total dissolved solids from output.
    fn set_output_tds_off(&self) -> Result<Self::DefaultReply>;
    /// Enable conductivity from output.
    fn set_output_conductivity_on(&self) -> Result<Self::DefaultReply>;
    /// Enable salinity from output.
    fn set_output_salinity_on(&self) -> Result<Self::DefaultReply>;
    /// Enable specific gravity from output.
    fn set_output_specific_gravity_on(&self) -> Result<Self::DefaultReply>;
    /// Enable total dissolved solids from output.
    fn set_output_tds_on(&self) -> Result<Self::DefaultReply>;
    /// Get the output string status.
    fn get_output_params(&self) -> Result<OutputStringStatus>;
    /// Set the probe type to `1.0`.
    fn set_probe_type_one(&self) -> Result<Self::DefaultReply>;
    /// Set the probe type to `0.1`.
    fn set_probe_type_point_one(&self) -> Result<Self::DefaultReply>;
    /// Set the probe type to `10`.
    fn set_probe_type_ten(&self) -> Result<Self::DefaultReply>;
    /// Get probe type status.
    fn get_probe_type_status(&self) -> Result<ProbeType>;
}
