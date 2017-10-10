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


// Use macro to define `TemperatureSensor`
sensor_i2cdev!(TemperatureSensor, "EZO-RTD Submersible Temperature Sensor");

impl TemperatureSensor {
    /// Change the EZO RTD chip to UART mode. WARNING: after using this command, the chip will not
    /// be available until it is put into I2C mode again. Read your chipset data-sheet for proper
    /// the procedure.
    pub fn set_uart_mode(&mut self, bps_rate: u32) -> Result<()> {
        let bps = BpsRate::parse_u32(bps_rate)?;
        let _cmd = Baud(bps)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Set the calibration temperature for the sensor.
    pub fn set_calibration_temperature(&mut self, t: f64) -> Result<()> {
        let _cmd = CalibrationTemperature(t)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Clear the device's calibration settings.
    pub fn set_calibration_clear(&mut self) -> Result<()> {
        let _cmd = CalibrationClear
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
}

impl TemperatureSensor {
    /// Get a summary of the number of calibration strings required to export the current sensor
    /// settings. It includes the number of lines and the total sum of exportable characters.
    pub fn get_export_info(&mut self) -> Result<ExportedInfo> {
        let info = ExportInfo
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(info)
    }

    /// Get a single calibration string from the device. This command needs to be called
    /// repeatedly, use the function `get_export_info()` to find out how many times.
    ///
    /// When the device has conclue
    pub fn get_export_line(&mut self) -> Result<Exported> {
        let export = Export
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(export)
    }

    /// Import a calibration string to the device.
    pub fn set_import_line(&mut self, import: String) -> Result<()> {
        let _import = Import(import)
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
    /// Set the sensor device to the factory settings.
    ///
    /// __NOTE:__ this will delete the settings of the device.
    pub fn set_factory_reset(&mut self) -> Result<()> {
        let _reset = Factory
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Set the device on Find mode. This will make the LED blink continuously until the device
    /// receives a new command.
    pub fn set_find_mode(&mut self) -> Result<()> {
        let _find = Find.run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Set a new I2C address on the device.
    ///
    /// __NOTE:__ using this command will make the current `self` obsolete. It is up to you to
    /// create a new `TemperatureSensor` that is properly configured.
    pub fn set_device_address(&mut self, address: u16) -> Result<()> {
        let _set = DeviceAddress(address)
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Get the general information about the sensor device.
    pub fn get_device_info(&mut self) -> Result<DeviceInfo> {
        let info = DeviceInformation
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(info)
    }
}

impl TemperatureSensor {
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
}

impl TemperatureSensor {
    /// Get the current sensor reading. Returns a `SensorReading` result.
    pub fn get_reading(&mut self) -> Result<SensorReading> {
        let reading = Reading
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

impl TemperatureSensor {
    /// Set the sensor chip to sleep.
    ///
    /// __NOTE:__ using this command will make the sensor device sleep until:
    ///
    /// 1.  it is woken up by writing a single byte to the device, or
    /// 2.   by sending __any__ valid command.
    pub fn set_sleep(&mut self) -> Result<()> {
        let _sleep = Sleep
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(())
    }
}

impl TemperatureSensor {
    /// Get the current status of the Temperature Sensor. Returns a `DeviceStatus` result.
    pub fn get_device_status(&mut self) -> Result<DeviceStatus> {
        let status = Status
            .run(&mut self.i2cdev)
            .chain_err(|| ErrorKind::SensorTrouble)?;
        Ok(status)
    }
}
