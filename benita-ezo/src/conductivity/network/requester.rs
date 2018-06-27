//! Client for Conductivity sensing.
use super::replies::*;
use super::requests::*;
use super::super::ConductivityAPI;

use common_ezo::EzoChipAPI;
use errors::*;
use network::{Endpoint, ReplyStatus, SocketRequest};

use zmq::Socket;

// Define the network client socket for sending requests to a
// `ConductivityResponder`.
network_socket! {
    ConductivityRequester,
    "Socket that makes requests to the Conductivity sensor socket."
}

impl EzoChipAPI for ConductivityRequester {
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

impl ConductivityAPI for ConductivityRequester {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    /// get the output string with sensor readings.
    fn get_reading(&self) -> Result<SensorReading> {
        let reply = Reading.send(self)?;
        Ok(reply)
    }

    /// Set the compensation temperature.
    fn set_compensation(&self, value: f64) -> Result<ReplyStatus> {
        let reply = CompensationSet(value).send(self)?;
        Ok(reply)
    }

    /// Get the current compensated temperature value.
    fn get_compensation(&self) -> Result<CompensationValue> {
        let reply = CompensationGet.send(self)?;
        Ok(reply)
    }

    /// Get the sensor's current calibration settings.
    fn get_calibration_status(&self) -> Result<CalibrationStatus> {
        let reply = CalibrationState.send(self)?;
        Ok(reply)
    }

    /// Set the value for dry calibration.
    fn set_calibration_dry(&self) -> Result<ReplyStatus> {
        let reply = CalibrationDry.send(self)?;
        Ok(reply)
    }

    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationHigh(t).send(self)?;
        Ok(reply)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationLow(t).send(self)?;
        Ok(reply)
    }

    /// Set the calibration single-point for the sensor.
    fn set_calibration_single(&self, t: f64) -> Result<ReplyStatus> {
        let reply = CalibrationOnePoint(t).send(self)?;
        Ok(reply)
    }

    /// Disable conductivity from output.
    fn set_output_conductivity_off(&self) -> Result<ReplyStatus> {
        let reply = OutputDisableConductivity.send(self)?;
        Ok(reply)
    }

    /// Disable salinity from output.
    fn set_output_salinity_off(&self) -> Result<ReplyStatus> {
        let reply = OutputDisableSalinity.send(self)?;
        Ok(reply)
    }

    /// Disable specific gravity from output.
    fn set_output_specific_gravity_off(&self) -> Result<ReplyStatus> {
        let reply = OutputDisableSpecificGravity.send(self)?;
        Ok(reply)
    }

    /// Disable total dissolved solids from output.
    fn set_output_tds_off(&self) -> Result<ReplyStatus> {
        let reply = OutputDisableTds.send(self)?;
        Ok(reply)
    }

    /// Enable conductivity from output.
    fn set_output_conductivity_on(&self) -> Result<ReplyStatus> {
        let reply = OutputEnableConductivity.send(self)?;
        Ok(reply)
    }

    /// Enable salinity from output.
    fn set_output_salinity_on(&self) -> Result<ReplyStatus> {
        let reply = OutputEnableSalinity.send(self)?;
        Ok(reply)
    }

    /// Enable specific gravity from output.
    fn set_output_specific_gravity_on(&self) -> Result<ReplyStatus> {
        let reply = OutputEnableSpecificGravity.send(self)?;
        Ok(reply)
    }

    /// Enable total dissolved solids from output.
    fn set_output_tds_on(&self) -> Result<ReplyStatus> {
        let reply = OutputEnableTds.send(self)?;
        Ok(reply)
    }

    /// Get the output string status.
    fn get_output_params(&self) -> Result<OutputStringStatus> {
        let reply = OutputState.send(self)?;
        Ok(reply)
    }

    /// Set the probe type to `1.0`.
    fn set_probe_type_one(&self) -> Result<ReplyStatus> {
        let reply = ProbeTypeOne.send(self)?;
        Ok(reply)
    }

    /// Set the probe type to `0.1`.
    fn set_probe_type_point_one(&self) -> Result<ReplyStatus> {
        let reply = ProbeTypePointOne.send(self)?;
        Ok(reply)
    }

    /// Set the probe type to `10`.
    fn set_probe_type_ten(&self) -> Result<ReplyStatus> {
        let reply = ProbeTypeTen.send(self)?;
        Ok(reply)
    }

    /// Get probe type status.
    fn get_probe_type_status(&self) -> Result<ProbeType> {
        let reply = ProbeTypeState.send(self)?;
        Ok(reply)
    }
}
