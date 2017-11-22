//! EZO RTD submersible temperature sensor. Command-API for the EZO RTD chipset.

/// Temperature I2C device `Error`, and `ErrorKind` definitions.
pub mod errors {
    error_chain!{}
}

use std::cell::RefCell;
use std::fmt;

use super::TemperatureAPI;
use super::command::*;
use super::response::*;

use config::SensorConfig;
use common_ezo::EzoChipAPI;
use devices::{I2CCommand, I2CResponse, SensorDevice};
use errors::*;
use network::ReplyStatus;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

pub use super::command as commands;
pub use super::response as responses;

// Use macro to define `TemperatureSensor`
device_i2cdev!(TemperatureSensor, "EZO-RTD Submersible Temperature Sensor");

impl EzoChipAPI for TemperatureSensor {
    type SensorError = Error;
    type SensorReply = ReplyStatus;

    sensor_commands!(device_common);
    sensor_commands!(calibration_common);
}

impl SensorDevice for TemperatureSensor  {
    type Error = Error;
}

impl TemperatureAPI for TemperatureSensor {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_commands!(calibration_status);
    sensor_commands!(reading);

    /// Set the calibration temperature for the sensor.
    fn set_calibration_temperature(&self, t: f64) -> Result<ReplyStatus> {
        let _cmd = CalibrationTemperature(t)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    fn set_data_logger_interval(&self, n: u32) -> Result<ReplyStatus> {
        let _set = DataloggerPeriod(n)
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Disable the data-logger.
    fn set_data_logger_off(&self) -> Result<ReplyStatus> {
        let _set = DataloggerDisable
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Get the current status of the data-logger.
    fn get_data_logger_status(&self) -> Result<DataLoggerStorageIntervalSeconds> {
        let interval = DataloggerInterval
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(interval)
    }

    /// Clear memory readings.
    fn set_memory_clear(&self) -> Result<ReplyStatus> {
        let _set = MemoryClear
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Recall the next memory reading on the stack.
    fn get_memory_recall(&self) -> Result<MemoryReading> {
        let reading = MemoryRecall
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }

    /// Recall the last memory reading on the stack.
    fn get_memory_recall_last(&self) -> Result<MemoryReading> {
        let reading = MemoryRecallLast
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }

    /// Set the current temperature scale to Celsius.
    fn set_scale_to_celsius(&self) -> Result<ReplyStatus> {
        let _set = ScaleCelsius
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the current temperature scale to Fahrenheit.
    fn set_scale_to_fahrenheit(&self) -> Result<ReplyStatus> {
        let _set = ScaleFahrenheit
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the current temperature scale to Kelvin.
    fn set_scale_to_kelvin(&self) -> Result<ReplyStatus> {
        let _set = ScaleKelvin
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(ReplyStatus::Ok)
    }

    /// Get the current temperature scale. Returns a `TemperatureScale` result.
    fn get_scale(&self) -> Result<TemperatureScale> {
        let scale = ScaleState
            .run(&mut self.i2cdev.borrow_mut())
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(scale)
    }
}
