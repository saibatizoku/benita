//! EZO RTD submersible temperature sensor. Command-API for the EZO RTD chipset.

pub mod commands {
    //! Commands from EZO RTD chipset.
    pub use ezo_rtd::command::{Baud, CalibrationClear, CalibrationState, CalibrationTemperature,
                               Command, DataloggerDisable, DataloggerInterval, DataloggerPeriod,
                               DeviceAddress, DeviceInformation, Export, ExportInfo, Factory,
                               Find, Import, LedOff, LedOn, LedState, MemoryClear, MemoryRecall,
                               MemoryRecallLast, ProtocolLockDisable, ProtocolLockEnable,
                               ProtocolLockState, Reading, ScaleCelsius, ScaleFahrenheit,
                               ScaleKelvin, ScaleState, Sleep, Status};
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

    /// Change the EZO RTD chip to UART mode. WARNING: after using this command, the chip will not
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

    pub fn set_calibration_temperature(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationTemperature(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_calibration_clear(&mut self) -> Result<()> {
        let _cmd = CalibrationClear
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn get_calibration_status(&mut self) -> Result<CalibrationStatus> {
        let cal = CalibrationState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(cal)
    }

    pub fn get_export_info(&mut self) -> Result<ExportedInfo> {
        let info = ExportInfo
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(info)
    }

    pub fn get_export_line(&mut self) -> Result<Exported> {
        let export = Export
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(export)
    }

    pub fn set_import_line(&mut self, import: String) -> Result<()> {
        let _import = Import(import)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_data_logger_interval(&mut self, n: u32) -> Result<()> {
        let _set = DataloggerPeriod(n)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_data_logger_off(&mut self) -> Result<()> {
        let _set = DataloggerDisable
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn get_data_logger_status(&mut self) -> Result<DataLoggerStorageIntervalSeconds> {
        let interval = DataloggerInterval
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(interval)
    }

    pub fn set_factory_reset(&mut self) -> Result<()> {
        let _reset = Factory
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_find_mode(&mut self) -> Result<()> {
        let _find = Find.run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_i2c_address(&mut self, address: u16) -> Result<()> {
        let _set = DeviceAddress(address)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn get_device_info(&mut self) -> Result<DeviceInfo> {
        let info = DeviceInformation
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(info)
    }

    pub fn set_led_off(&mut self) -> Result<()> {
        let _set = LedOff
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_led_on(&mut self) -> Result<()> {
        let _set = LedOn
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn get_led_status(&mut self) -> Result<LedStatus> {
        let status = LedState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }

    pub fn set_memory_clear(&mut self) -> Result<()> {
        let _set = MemoryClear
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn get_memory_recall(&mut self) -> Result<MemoryReading> {
        let reading = MemoryRecall
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }

    pub fn get_memory_recall_last(&mut self) -> Result<MemoryReading> {
        let reading = MemoryRecallLast
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }

    pub fn set_protocol_lock_off(&mut self) -> Result<()> {
        let _set = ProtocolLockDisable
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn set_protocol_lock_on(&mut self) -> Result<()> {
        let _set = ProtocolLockEnable
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    pub fn get_protocol_lock_status(&mut self) -> Result<ProtocolLockStatus> {
        let status = ProtocolLockState
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }

    /// Get the current sensor reading. Returns a `SensorReading` result.
    pub fn get_reading(&mut self) -> Result<SensorReading> {
        let reading = Reading
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(reading)
    }

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

    /// Set the sensor chip to sleep.
    pub fn set_sleep(&mut self) -> Result<()> {
        let _sleep = Sleep
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }

    /// Get the current status of the Temperature Sensor. Returns a `DeviceStatus` result.
    pub fn get_status(&mut self) -> Result<DeviceStatus> {
        let status = Status
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }
}
