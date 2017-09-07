//! EZO RTD submersible temperature sensor. Command-API for the EZO RTD chipset.

pub mod commands {
    //! Commands from EZO RTD chipset.
    pub use ezo_rtd::command::{Baud, CalibrationState, CalibrationTemperature, CalibrationClear, DataloggerPeriod, DataloggerDisable, DataloggerInterval, DeviceInformation, Export, ExportInfo, Factory, Find, Import, LedOff, LedOn, LedState, MemoryClear, MemoryRecall, MemoryRecallLast, ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState, Reading, ScaleCelsius, ScaleFahrenheit,ScaleKelvin, ScaleState, Sleep, Status};
}

pub mod responses {
    //! Responses from EZO RTD chipset.
    pub use ezo_rtd::response::{CalibrationStatus, DataLoggerStorageIntervalSeconds, DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus, MemoryReading, ProtocolLockStatus, SensorReading, Temperature, TemperatureScale};
}

use errors::*;
use i2cdev::linux::LinuxI2CDevice;
use ezo_common::BpsRate;

use self::commands::*;
use self::responses::*;

/// EZO-RTD Submersible Temperature Sensor
pub struct TemperatureSensor {
    i2cdev: LinuxI2CDevice,
}

impl TemperatureSensor {
    /// Creates a new handle for the Temperature Sensor connected
    /// at the designated path and address.
    pub fn new(i2c_path: &str, device_address: u16) -> Result<TemperatureSensor> {
        let i2cdev = LinuxI2CDevice::new(i2c_path, device_address)
            .chain_err(|| "Could not open the specified I2C device")?;
        Ok(TemperatureSensor { i2cdev: i2cdev })
    }

    /// Get the current temperature scale
    pub fn get_scale(&mut self) -> Result<TemperatureScale> {
        let scale = ScaleState.run(&mut self.i2cdev)?;
        Ok(scale)
    }

    /// Get the current status of the Temperature Sensor
    pub fn get_status(&mut self) -> Result<DeviceStatus> {
        let status = Status.run(&mut self.i2cdev)?;
        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_rtd_sensor() {
        assert!(true);
    }
}
