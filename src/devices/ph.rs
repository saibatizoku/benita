//! EZO PH submersible pH sensor. Command-API for the EZO PH chipset.

pub mod commands {
    //! Commands from EZO PH chipset.
    pub use ezo_ph::command::Baud;
    pub use ezo_ph::command::Command;
    pub use ezo_ph::command::{CalibrationClear, CalibrationHigh, CalibrationLow, CalibrationMid,
                              CalibrationState};
    pub use ezo_ph::command::{CompensatedTemperatureValue as CompensationGet, DeviceAddress,
                              TemperatureCompensation as CompensationSet};
    pub use ezo_ph::command::{DeviceInformation, Factory, Find, Reading, Sleep, Status};
    pub use ezo_ph::command::{Export, ExportInfo, Import};
    pub use ezo_ph::command::{LedOff, LedOn, LedState};
    pub use ezo_ph::command::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
    pub use ezo_ph::command::Slope;
}

pub mod responses {
    //! Responses from EZO PH chipset.
    pub use ezo_ph::response::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus,
                               Exported, ExportedInfo, LedStatus, ProbeSlope, ProtocolLockStatus,
                               SensorReading};
}

pub mod errors {
    error_chain! {
    }
}

use std::cell::RefCell;
use std::fmt;

use self::commands::*;
use self::responses::*;

use api::ph::PhAPI;
use config::SensorConfig;
use super::errors::*;
use network::common::ReplyStatus;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

// Use macro to define `PhSensor`
device_i2cdev!(PhSensor, "EZO-EC Submersible pH Sensor.");

impl PhAPI for PhSensor {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_commands!(device_common);

    sensor_commands!(calibration_common);

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

    /// Set the value for mid-point calibration.
    fn set_calibration_mid(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationMid(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    sensor_commands!(temperature_compensation);

    /// Get the current slope of the pH Sensor.
    ///
    /// Returns a `ProbeSlope` result.
    fn get_slope(&self) -> Result<ProbeSlope> {
        let slope = Slope
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(slope)
    }
}
