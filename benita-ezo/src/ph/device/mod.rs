//! EZO PH submersible pH sensor. Command-API for the EZO PH chipset.
use std::cell::RefCell;
use std::fmt;

use super::command::*;
use super::response::*;
use super::PhAPI;

use common_ezo::EzoChipAPI;
use config::SensorConfig;
use devices::SensorDevice;
use errors::*;
use network::ReplyStatus;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

pub use super::command as commands;
pub use super::response as responses;

// Use macro to define `PhSensor`
device_i2cdev!(PhSensor, "EZO-EC Submersible pH Sensor.");

impl SensorDevice<PhSensor> for PhSensor {
    type Error = Error;

    fn i2c_mut(&self) -> ::std::cell::RefMut<LinuxI2CDevice> {
        self.i2cdev.borrow_mut()
    }
}

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
            .context(ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationLow(t)
            .run(&mut self.i2cdev.borrow_mut())
            .context(ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the value for mid-point calibration.
    fn set_calibration_mid(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationMid(t)
            .run(&mut self.i2cdev.borrow_mut())
            .context(ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    sensor_commands!(temperature_compensation);

    /// Get the current slope of the pH Sensor.
    ///
    /// Returns a `ProbeSlope` result.
    fn get_slope(&self) -> Result<ProbeSlope> {
        let slope = Slope
            .run(&mut self.i2cdev.borrow_mut())
            .context(ErrorKind::SensorTrouble)?;
        Ok(slope)
    }
}
