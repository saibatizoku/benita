//! Common sensor command methods
#[macro_export]
macro_rules! sensor_commands {
    ( calibration_common ) => {
        /// Clear the sensor's calibration settings.
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
    };
    ( device_common ) => {
        /// Change the sensor to UART mode.
        ///
        /// __WARNING:__ after using this command, the chip will not be available
        /// until it is put into I2C mode again. Read your chipset data-sheet for proper
        /// the procedure.
        pub fn set_uart_mode(&mut self, bps_rate: u32) -> Result<()> {
            let bps = BpsRate::parse_u32(bps_rate)?;
            let _cmd = Baud(bps)
                .run(&mut self.i2cdev)
                .chain_err(|| ErrorKind::SensorTrouble)?;
            Ok(())
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

        /// Get the general information about the sensor device.
        pub fn get_device_info(&mut self) -> Result<DeviceInfo> {
            let info = DeviceInformation
                .run(&mut self.i2cdev)
                .chain_err(|| ErrorKind::SensorTrouble)?;
            Ok(info)
        }

        /// Get the current status of the sensor device.
        ///
        /// Returns a `DeviceStatus` result.
        pub fn get_device_status(&mut self) -> Result<DeviceStatus> {
            let status = Status
                .run(&mut self.i2cdev)
                .chain_err(|| ErrorKind::SensorTrouble)?;
            Ok(status)
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
        /// create a new sensor that is properly configured.
        pub fn set_device_address(&mut self, address: u16) -> Result<()> {
            let _set = DeviceAddress(address)
                .run(&mut self.i2cdev)
                .chain_err(|| ErrorKind::SensorTrouble)?;
            Ok(())
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
        pub fn get_reading(&mut self) -> Result<SensorReading> {
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
    };
    ( temperature_compensation ) => {
        /// Set the compensation temperature.
        pub fn set_compensation_temperature(&mut self, value: f64) -> Result<()> {
            let _cmd = TemperatureCompensation(value)
                .run(&mut self.i2cdev)
                .chain_err(|| ErrorKind::SensorTrouble)?;
            Ok(())
        }

        /// Get the current compensated temperature value.
        pub fn get_compensated_temperature_value(&mut self) -> Result<CompensationValue> {
            let value = CompensatedTemperatureValue
                .run(&mut self.i2cdev)
                .chain_err(|| ErrorKind::SensorTrouble)?;
            Ok(value)
        }
    };
}

