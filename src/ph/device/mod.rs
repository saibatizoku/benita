//! EZO PH submersible pH sensor. Command-API for the EZO PH chipset.

/// pH I2C device `Error`, and `ErrorKind` definitions.
pub mod errors {
    error_chain! {
    }
}

use std::cell::RefCell;
use std::fmt;

use super::PhAPI;
use super::command::*;
use super::errors::*;
use super::response::*;

use common_ezo::EzoChipAPI;
use config::SensorConfig;
use network::ReplyStatus;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

pub use super::command as commands;
pub use super::response as responses;

// Use macro to define `PhSensor`
device_i2cdev!(PhSensor, "EZO-EC Submersible pH Sensor.");

impl EzoChipAPI for PhSensor {
    type SensorError = Error;
    type SensorReply = ReplyStatus;

    sensor_commands!(device_common);
    sensor_commands!(calibration_common);
}

impl PhAPI for PhSensor {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_commands!(calibration_status);
    sensor_commands!(reading);

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
