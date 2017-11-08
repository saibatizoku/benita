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

use std::cell::RefCell;
use std::fmt;

use config::SensorConfig;
use errors::*;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

use self::commands::*;

use self::responses::*;

// Use macro to define `PhSensor`
device_i2cdev!(PhSensor, "EZO-EC Submersible pH Sensor.");

impl PhSensor {
    sensor_commands!(device_common);
}

impl PhSensor {
    sensor_commands!(calibration_common);

    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&self, t: f64) -> Result<()> {
        let _cmd = CalibrationHigh(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&self, t: f64) -> Result<()> {
        let _cmd = CalibrationLow(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the value for mid-point calibration.
    pub fn set_calibration_mid(&self, t: f64) -> Result<()> {
        let _cmd = CalibrationMid(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl PhSensor {
    sensor_commands!(temperature_compensation);
}

impl PhSensor {
    /// Get the current slope of the pH Sensor.
    ///
    /// Returns a `ProbeSlope` result.
    pub fn get_slope(&self) -> Result<ProbeSlope> {
        let slope = Slope
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(slope)
    }
}
