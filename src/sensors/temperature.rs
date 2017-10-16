//! EZO RTD submersible temperature sensor. Command-API for the EZO RTD chipset.

pub mod commands {
    //! Commands from EZO RTD chipset.
    pub use ezo_rtd::command::Baud;
    pub use ezo_rtd::command::Command;
    pub use ezo_rtd::command::{CalibrationClear, CalibrationState, CalibrationTemperature};
    pub use ezo_rtd::command::{DataloggerDisable, DataloggerInterval, DataloggerPeriod};
    pub use ezo_rtd::command::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep,
                               Status};
    pub use ezo_rtd::command::{Export, ExportInfo, Import};
    pub use ezo_rtd::command::{LedOff, LedOn, LedState};
    pub use ezo_rtd::command::{MemoryClear, MemoryRecall, MemoryRecallLast};
    pub use ezo_rtd::command::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
    pub use ezo_rtd::command::{ScaleCelsius, ScaleFahrenheit, ScaleKelvin, ScaleState};
}

pub mod responses {
    //! Responses from EZO RTD chipset.
    pub use ezo_rtd::response::{CalibrationStatus, DataLoggerStorageIntervalSeconds, DeviceInfo,
                                DeviceStatus, Exported, ExportedInfo, LedStatus, MemoryReading,
                                ProtocolLockStatus, SensorReading, Temperature, TemperatureScale};
}

use errors::*;
use i2cdev::linux::LinuxI2CDevice;
use ezo_common::BpsRate;

use self::commands::*;
use self::responses::*;


// Use macro to define `TemperatureSensor`
sensor_i2cdev!(TemperatureSensor, "EZO-RTD Submersible Temperature Sensor");

impl TemperatureSensor {
    sensor_commands!(device_common);
}

impl TemperatureSensor {
    sensor_commands!(calibration_common);

    /// Set the calibration temperature for the sensor.
    pub fn set_calibration_temperature(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationTemperature(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Set the data logger interval, `n`.
    ///
    /// The device will take readings and save them to memory at the given interval.
    pub fn set_data_logger_interval(&mut self, n: u32) -> Result<()> {
        let _set = DataloggerPeriod(n)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Disable the data-logger.
    pub fn set_data_logger_off(&mut self) -> Result<()> {
        let _set = DataloggerDisable
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the current status of the data-logger.
    pub fn get_data_logger_status(&mut self) -> Result<DataLoggerStorageIntervalSeconds> {
        let interval = DataloggerInterval
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(interval)
    }
}

impl TemperatureSensor {
    /// Clear memory readings.
    pub fn set_memory_clear(&mut self) -> Result<()> {
        let _set = MemoryClear
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Recall the next memory reading on the stack.
    pub fn get_memory_recall(&mut self) -> Result<MemoryReading> {
        let reading = MemoryRecall
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }

    /// Recall the last memory reading on the stack.
    pub fn get_memory_recall_last(&mut self) -> Result<MemoryReading> {
        let reading = MemoryRecallLast
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }
}

impl TemperatureSensor {
    /// Set the current temperature scale to Celsius.
    pub fn set_scale_to_celsius(&mut self) -> Result<()> {
        let _set = ScaleCelsius
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the current temperature scale to Fahrenheit.
    pub fn set_scale_to_fahrenheit(&mut self) -> Result<()> {
        let _set = ScaleFahrenheit
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the current temperature scale to Kelvin.
    pub fn set_scale_to_kelvin(&mut self) -> Result<()> {
        let _set = ScaleKelvin
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the current temperature scale. Returns a `TemperatureScale` result.
    pub fn get_scale(&mut self) -> Result<TemperatureScale> {
        let scale = ScaleState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(scale)
    }
}
