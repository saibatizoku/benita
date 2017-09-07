//! EZO EC submersible electrical conductivity sensor. Command-API for the EZO EC chipset.

pub mod commands {
    //! Commands from EZO EC chipset.
    pub use ezo_ec::command::{Baud, CalibrationClear, CalibrationDry, CalibrationHigh,
                              CalibrationLow, CalibrationOnePoint, CalibrationState, Command,
                              CompensatedTemperatureValue, DeviceAddress, DeviceInformation,
                              Export, ExportInfo, Factory, Find, Import, LedOff, LedOn, LedState,
                              OutputDisableConductivity, OutputDisableSalinity,
                              OutputDisableSpecificGravity, OutputDisableTds,
                              OutputEnableConductivity, OutputEnableSalinity,
                              OutputEnableSpecificGravity, OutputEnableTds, OutputState,
                              ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen,
                              ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState, Reading,
                              Sleep, Status, TemperatureCompensation};
}

pub mod responses {
    //! Responses from EZO EC chipset.
    pub use ezo_ec::response::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus, OutputStringStatus, ParameterStatus, ProbeReading, ProbeType, ProtocolLockStatus};
}

use errors::*;
use i2cdev::linux::LinuxI2CDevice;
use ezo_common::BpsRate;

use self::commands::*;
use self::responses::*;

/// EZO-EC Submersible Electrical Conductivity Sensor
pub struct ConductivitySensor {
    i2cdev: LinuxI2CDevice,
}

impl ConductivitySensor {
    /// Creates a new handle for the Electrical Conductivity Sensor connected
    /// at the designated path and address.
    pub fn new(i2c_path: &str, device_address: u16) -> Result<ConductivitySensor> {
        let i2cdev = LinuxI2CDevice::new(i2c_path, device_address)
            .chain_err(|| "Could not open the specified I2C device")?;
        Ok(ConductivitySensor { i2cdev: i2cdev })
    }

    /// Change the EZO EC chip to UART mode. WARNING: after using this command, the chip will not
    /// be available until it is put into I2C mode again. Read your chipset data-sheet for proper
    /// the procedure.
    pub fn set_uart_mode(&mut self, bps_rate: u32) -> Result<()> {
        let bps = match bps_rate {
            300 => BpsRate::Bps300,
            1200 => BpsRate::Bps1200,
            2400 => BpsRate::Bps2400,
            9600 => BpsRate::Bps9600,
            19200 => BpsRate::Bps19200,
            38400 => BpsRate::Bps38400,
            57600 => BpsRate::Bps57600,
            115200 => BpsRate::Bps115200,
            _ => return Err(ErrorKind::SensorTrouble.into()),
        };
        let _cmd = Baud(bps)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Clear the sensor's calibration settings.
    pub fn set_calibration_clear(&mut self) -> Result<()> {
        let _cmd = CalibrationClear
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the value for dry calibration.
    pub fn set_calibration_dry(&mut self) -> Result<()> {
        let _cmd = CalibrationClear
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the calibration temperature for the sensor.
    pub fn set_calibration_high(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationTemperature(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the sensor's current calibration settings.
    pub fn get_calibration_status(&mut self) -> Result<CalibrationStatus> {
        let cal = CalibrationState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(cal)
    }

    /// Get a summary of the number of calibration strings required to export the current sensor
    /// settings. It includes the number of lines and the total sum of exportable characters.
    pub fn get_export_info(&mut self) -> Result<ExportedInfo> {
        let info = ExportInfo
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(info)
    }

    /// Get a single calibration string from the sensor. This command needs to be called
    /// repeatedly, use the function `get_export_info()` to find out how many times.
    pub fn get_export_line(&mut self) -> Result<Exported> {
        let export = Export
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(export)
    }

    /// Import a calibration string to the sensor.
    pub fn set_import_line(&mut self, import: String) -> Result<()> {
        let _import = Import(import)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the sensor to the factory settings.
    ///
    /// __NOTE:__ this will delete the settings of the sensor.
    pub fn set_factory_reset(&mut self) -> Result<()> {
        let _reset = Factory
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the sensor on Find mode. This will make the LED blink continuously until the sensor
    /// receives a new command.
    pub fn set_find_mode(&mut self) -> Result<()> {
        let _find = Find.run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set a new I2C address on the sensor.
    ///
    /// __NOTE:__ using this command will make the current `self` obsolete. It is up to you to
    /// create a new `ConductivitySensor` that is properly configured.
    pub fn set_i2c_address(&mut self, address: u16) -> Result<()> {
        let _set = DeviceAddress(address)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the general information about the sensor device.
    pub fn get_device_info(&mut self) -> Result<DeviceInfo> {
        let info = DeviceInformation
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(info)
    }

    /// Turn off the LED.
    pub fn set_led_off(&mut self) -> Result<()> {
        let _set = LedOff
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Turn on the LED.
    pub fn set_led_on(&mut self) -> Result<()> {
        let _set = LedOn
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the current status of the LED.
    pub fn get_led_status(&mut self) -> Result<LedStatus> {
        let status = LedState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }

    /// Set the lock off for the I2C protocol mode.
    pub fn set_protocol_lock_off(&mut self) -> Result<()> {
        let _set = ProtocolLockDisable
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Set the lock on for the I2C protocol mode.
    pub fn set_protocol_lock_on(&mut self) -> Result<()> {
        let _set = ProtocolLockEnable
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the I2C protocol mode status.
    pub fn get_protocol_lock_status(&mut self) -> Result<ProtocolLockStatus> {
        let status = ProtocolLockState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }

    /// Get the current sensor reading. Returns a `SensorReading` result.
    pub fn get_reading(&mut self) -> Result<ProbeReading> {
        let reading = Reading
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }


    /// Set the sensor chip to sleep.
    ///
    /// __NOTE:__ using this command will make the sensor device sleep until:
    ///
    /// 1.  it is woken up by writing a single byte to the sensor, or
    /// 2.   by sending __any__ valid command.
    pub fn set_sleep(&mut self) -> Result<()> {
        let _sleep = Sleep
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the current status of the Electrical Conductivity Sensor. Returns a `DeviceStatus` result.
    pub fn get_status(&mut self) -> Result<DeviceStatus> {
        let status = Status
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }
}
