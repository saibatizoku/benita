//! EZO EC submersible electrical conductivity sensor. Command-API for the EZO EC chipset.

pub mod commands {
    //! Commands from EZO EC chipset.
    pub use ezo_ec::command::Command;
    pub use ezo_ec::command::Baud;
    pub use ezo_ec::command::{CalibrationClear, CalibrationDry, CalibrationHigh, CalibrationLow,
                              CalibrationOnePoint, CalibrationState};
    pub use ezo_ec::command::{CompensatedTemperatureValue as CompensationGet,
                              TemperatureCompensation as CompensationSet};
    pub use ezo_ec::command::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep,
                              Status};
    pub use ezo_ec::command::{Export, ExportInfo, Import};
    pub use ezo_ec::command::{LedOff, LedOn, LedState};
    pub use ezo_ec::command::{OutputDisableConductivity, OutputDisableSalinity,
                              OutputDisableSpecificGravity, OutputDisableTds,
                              OutputEnableConductivity, OutputEnableSalinity,
                              OutputEnableSpecificGravity, OutputEnableTds, OutputState};
    pub use ezo_ec::command::{ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen};
    pub use ezo_ec::command::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
}

pub mod responses {
    //! Responses from EZO EC chipset.
    pub use ezo_ec::response::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus,
                               Exported, ExportedInfo, LedStatus, OutputStringStatus,
                               ProbeReading, ProbeType, ProtocolLockStatus};
}

use std::cell::RefCell;
use std::fmt;

use self::commands::*;
use self::responses::*;

use api::conductivity::ConductivityAPI;
use config::SensorConfig;
use errors::*;
use network::common::ReplyStatus;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

pub type SensorReading = ProbeReading;

// Use macro to define `ConductivitySensor`
device_i2cdev!(
    ConductivitySensor,
    "EZO-EC Submersible Electrical Conductivity Sensor."
);

impl ConductivityAPI for ConductivitySensor {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_commands!(device_common);

    sensor_commands!(calibration_common);

    /// Set the value for dry calibration.
    fn set_calibration_dry(&self) -> Result<ReplyStatus> {
        let _cmd = CalibrationDry
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationHigh(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationLow(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration single-point for the sensor.
    fn set_calibration_single(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationOnePoint(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    sensor_commands!(temperature_compensation);

    /// Disable conductivity from output.
    fn set_output_conductivity_off(&self) -> Result<ReplyStatus> {
        let _set = OutputDisableConductivity
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Disable salinity from output.
    fn set_output_salinity_off(&self) -> Result<ReplyStatus> {
        let _set = OutputDisableSalinity
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Disable specific gravity from output.
    fn set_output_specific_gravity_off(&self) -> Result<ReplyStatus> {
        let _set = OutputDisableSpecificGravity
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Disable total dissolved solids from output.
    fn set_output_tds_off(&self) -> Result<ReplyStatus> {
        let _set = OutputDisableTds
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Enable conductivity from output.
    fn set_output_conductivity_on(&self) -> Result<ReplyStatus> {
        let _set = OutputEnableConductivity
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Enable salinity from output.
    fn set_output_salinity_on(&self) -> Result<ReplyStatus> {
        let _set = OutputEnableSalinity
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Enable specific gravity from output.
    fn set_output_specific_gravity_on(&self) -> Result<ReplyStatus> {
        let _set = OutputEnableSpecificGravity
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Enable total dissolved solids from output.
    fn set_output_tds_on(&self) -> Result<ReplyStatus> {
        let _set = OutputEnableTds
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Get the output string status.
    fn get_output_params(&self) -> Result<OutputStringStatus> {
        let status = OutputState
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }

    /// Set the probe type to `1.0`.
    fn set_probe_type_one(&self) -> Result<ReplyStatus> {
        let _set = ProbeTypeOne
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the probe type to `0.1`.
    fn set_probe_type_point_one(&self) -> Result<ReplyStatus> {
        let _set = ProbeTypePointOne
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the probe type to `10`.
    fn set_probe_type_ten(&self) -> Result<ReplyStatus> {
        let _set = ProbeTypeTen
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Get probe type status.
    fn get_probe_type_status(&self) -> Result<ProbeType> {
        let status = ProbeTypeState
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }
}
