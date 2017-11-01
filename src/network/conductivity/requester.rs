//! Client for Conductivity sensing.
use errors::*;
use network::{Endpoint, SocketReply, SocketRequest};
use network::conductivity::requests::*;

use neuras;

// Define the network client socket for sending requests to a
// `ConductivityResponder`.
network_socket! {
    ConductivityRequester,
    "Socket that makes requests to the Conductivity sensor socket."
}

impl ConductivityRequester {
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

impl ConductivityRequester {
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

impl ConductivityRequester {
    /// Clear the sensor's calibration settings.
    pub fn set_calibration_clear(&self) -> Result<String> {
        let reply = CalibrationClear.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get the sensor's current calibration settings.
    pub fn get_calibration_status(&self) -> Result<String> {
        let reply = CalibrationState.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the value for dry calibration.
    pub fn set_calibration_dry(&self) -> Result<String> {
        let reply = CalibrationDry.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&self, t: f64) -> Result<String> {
        let reply = CalibrationHigh(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&self, t: f64) -> Result<String> {
        let reply = CalibrationLow(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the calibration single-point for the sensor.
    pub fn set_calibration_single(&self, t: f64) -> Result<String> {
        let reply = CalibrationOnePoint(t).send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl ConductivityRequester {
    /// Disable conductivity from output.
    pub fn set_output_conductivity_off(&self) -> Result<String> {
        let reply = OutputDisableConductivity.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Disable salinity from output.
    pub fn set_output_salinity_off(&self) -> Result<String> {
        let reply = OutputDisableSalinity.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Disable specific gravity from output.
    pub fn set_output_specific_gravity_off(&self) -> Result<String> {
        let reply = OutputDisableSpecificGravity.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Disable total dissolved solids from output.
    pub fn set_output_tds_off(&self) -> Result<String> {
        let reply = OutputDisableTds.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Enable conductivity from output.
    pub fn set_output_conductivity_on(&self) -> Result<String> {
        let reply = OutputEnableConductivity.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Enable salinity from output.
    pub fn set_output_salinity_on(&self) -> Result<String> {
        let reply = OutputEnableSalinity.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Enable specific gravity from output.
    pub fn set_output_specific_gravity_on(&self) -> Result<String> {
        let reply = OutputEnableSpecificGravity.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Enable total dissolved solids from output.
    pub fn set_output_tds_on(&self) -> Result<String> {
        let reply = OutputEnableTds.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get the output string status.
    pub fn get_output_string_status(&self) -> Result<String> {
        let reply = OutputState.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}

impl ConductivityRequester {
    /// Set the probe type to `1.0`.
    pub fn set_probe_type_one(&self) -> Result<String> {
        let reply = ProbeTypeOne.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the probe type to `0.1`.
    pub fn set_probe_type_point_one(&self) -> Result<String> {
        let reply = ProbeTypePointOne.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Set the probe type to `10`.
    pub fn set_probe_type_ten(&self) -> Result<String> {
        let reply = ProbeTypeTen.send_to(self)?;
        Ok(reply.to_reply_string())
    }

    /// Get probe type status.
    pub fn get_probe_type_status(&self) -> Result<String> {
        let reply = ProbeTypeState.send_to(self)?;
        Ok(reply.to_reply_string())
    }
}
