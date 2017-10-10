//! EZO PH submersible pH sensor. Command-API for the EZO PH chipset.

pub mod commands {
    //! Commands from EZO PH chipset.
    pub use ezo_ph::command::*;
}

pub mod responses {
    //! Responses from EZO PH chipset.
    pub use ezo_ph::response::*;
}

use errors::*;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

use self::commands::{Baud, CalibrationClear, CalibrationHigh, CalibrationLow, CalibrationMid,
                     CalibrationState, Command, CompensatedTemperatureValue, DeviceAddress,
                     DeviceInformation, Export, ExportInfo, Factory, Find, Import, LedOff, LedOn,
                     LedState, ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState,
                     Reading, Sleep, Slope, Status, TemperatureCompensation};

use self::responses::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus, Exported,
                      ExportedInfo, LedStatus, ProbeSlope, ProtocolLockStatus, SensorReading};

// Use macro to define `PhSensor`
sensor_i2cdev!(PhSensor, "EZO-EC Submersible pH Sensor.");

impl PhSensor {
    sensor_commands!(device_common);
}

impl PhSensor {
    sensor_commands!(calibration_common);

    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationHigh(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationLow(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the value for mid-point calibration.
    pub fn set_calibration_mid(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationMid(t)
            .run(&mut self.i2cdev)
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
    pub fn get_slope(&mut self) -> Result<ProbeSlope> {
        let slope = Slope
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(slope)
    }
}
