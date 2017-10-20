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

use errors::*;

use ezo_common::BpsRate;
use i2cdev::linux::LinuxI2CDevice;

use self::commands::*;

use self::responses::*;

pub type SensorReading = ProbeReading;

// Use macro to define `ConductivitySensor`
sensor_i2cdev!(
    ConductivitySensor,
    "EZO-EC Submersible Electrical Conductivity Sensor."
);

impl ConductivitySensor {
    sensor_commands!(device_common);
}

impl ConductivitySensor {
    sensor_commands!(calibration_common);

    /// Set the value for dry calibration.
    pub fn set_calibration_dry(&mut self) -> Result<()> {
        let _cmd = CalibrationDry
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

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

    /// Set the calibration single-point for the sensor.
    pub fn set_calibration_single(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationOnePoint(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl ConductivitySensor {
    sensor_commands!(temperature_compensation);
}

impl ConductivitySensor {
    /// Disable conductivity from output.
    pub fn set_output_conductivity_off(&mut self) -> Result<()> {
        let _set = OutputDisableConductivity
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Disable salinity from output.
    pub fn set_output_salinity_off(&mut self) -> Result<()> {
        let _set = OutputDisableSalinity
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Disable specific gravity from output.
    pub fn set_output_specific_gravity_off(&mut self) -> Result<()> {
        let _set = OutputDisableSpecificGravity
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Disable total dissolved solids from output.
    pub fn set_output_tds_off(&mut self) -> Result<()> {
        let _set = OutputDisableTds
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Enable conductivity from output.
    pub fn set_output_conductivity_on(&mut self) -> Result<()> {
        let _set = OutputEnableConductivity
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Enable salinity from output.
    pub fn set_output_salinity_on(&mut self) -> Result<()> {
        let _set = OutputEnableSalinity
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Enable specific gravity from output.
    pub fn set_output_specific_gravity_on(&mut self) -> Result<()> {
        let _set = OutputEnableSpecificGravity
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Enable total dissolved solids from output.
    pub fn set_output_tds_on(&mut self) -> Result<()> {
        let _set = OutputEnableTds
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the output string status.
    pub fn get_output_string_status(&mut self) -> Result<OutputStringStatus> {
        let status = OutputState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }
}

impl ConductivitySensor {
    /// Set the probe type to `1.0`.
    pub fn set_probe_type_one(&mut self) -> Result<()> {
        let _set = ProbeTypeOne
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the probe type to `0.1`.
    pub fn set_probe_type_point_one(&mut self) -> Result<()> {
        let _set = ProbeTypePointOne
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the probe type to `10`.
    pub fn set_probe_type_ten(&mut self) -> Result<()> {
        let _set = ProbeTypeTen
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get probe type status.
    pub fn get_probe_type_status(&mut self) -> Result<ProbeType> {
        let status = ProbeTypeState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }
}
