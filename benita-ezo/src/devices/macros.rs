/// Create and define a sensor available through `i2cdev`.
#[macro_export]
macro_rules! device_i2cdev {
    // Name identifier and documentation for the new I2C sensor struct.
    ($name:ident, $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            path: String,
            address: u16,
            pub i2cdev: RefCell<LinuxI2CDevice>,
        }

        impl $name {
            /// Creates a new handle for the I2C Sensor connected
            /// at the designated path and address.
            ///
            /// This device uses a file-descriptor through `i2cdev`. To use it, the path
            /// to the I2C bus, and the `u16` address location, are needed.
            pub fn new(path: &str, address: u16) -> Result<$name> {
                let i2cdev = LinuxI2CDevice::new(path, address).context(ErrorKind::SensorTrouble)?;
                let path = path.to_string();
                Ok($name {
                    path,
                    address,
                    i2cdev: RefCell::new(i2cdev),
                })
            }

            /// Create a new I2C sensor instance from `SensorConfig`.
            pub fn from_config(config: SensorConfig) -> Result<$name> {
                let config_path = match config.path.to_str() {
                    Some(path) => path,
                    _ => return Err(ErrorKind::InvalidDevice)?,
                };
                $name::new(config_path, config.address)
            }

            pub fn device_mut(&self) -> ::std::cell::RefMut<LinuxI2CDevice> {
                self.i2cdev.borrow_mut()
            }

            /// Change the sensor to UART mode.
            ///
            /// __WARNING:__ after using this command, the chip will not be available
            /// until it is put into I2C mode again. Read your chipset data-sheet for proper
            /// the procedure.
            pub fn set_uart_mode(&self, bps_rate: u32) -> Result<ReplyStatus> {
                let bps = BpsRate::parse_u32(bps_rate).context(ErrorKind::IncorrectBps)?;
                let _cmd = Baud(bps)
                    .run(&mut self.i2cdev.borrow_mut())
                    .context(ErrorKind::SensorTrouble)?;
                Ok(ReplyStatus::Ok)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{} {{ ADDRESS {} @ {}}}",
                    stringify!($name),
                    self.address,
                    self.path
                )
            }
        }
    };
}

/// Common sensor command methods
macro_rules! sensor_commands {
    ( calibration_common ) => {
        /// Clear the sensor's calibration settings.
        fn set_calibration_clear(&self) -> Result<ReplyStatus> {
            let _cmd = CalibrationClear
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }
    };

    ( calibration_status ) => {
        /// Get the sensor's current calibration settings.
        fn get_calibration_status(&self) -> Result<CalibrationStatus> {
            let cal = CalibrationState
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(cal)
        }
    };

    ( device_common ) => {
        /// Get a summary of the number of calibration strings required to export the current sensor
        /// settings. It includes the number of lines and the total sum of exportable characters.
        fn get_export_info(&self) -> Result<ExportedInfo> {
            let info = ExportInfo
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(info)
        }

        /// Get a single calibration string from the sensor. This command needs to be called
        /// repeatedly, use the function `get_export_info()` to find out how many times.
        fn get_export_line(&self) -> Result<Exported> {
            let export = Export
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(export)
        }

        /// Import a calibration string to the sensor.
        fn set_import_line(&self, import: &str) -> Result<ReplyStatus> {
            let _import = Import(import.to_string())
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Get the general information about the sensor device.
        fn get_device_info(&self) -> Result<DeviceInfo> {
            let info = DeviceInformation
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(info)
        }

        /// Get the current status of the sensor device.
        ///
        /// Returns a `DeviceStatus` result.
        fn get_device_status(&self) -> Result<DeviceStatus> {
            let status = Status
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(status)
        }

        /// Set the sensor to the factory settings.
        ///
        /// __NOTE:__ this will delete the settings of the sensor.
        fn set_factory_reset(&self) -> Result<ReplyStatus> {
            let _reset = Factory
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Set the sensor on Find mode. This will make the LED blink continuously until the sensor
        /// receives a new command.
        fn set_find_mode(&self) -> Result<ReplyStatus> {
            let _find = Find.run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Set a new I2C address on the sensor.
        ///
        /// __NOTE:__ using this command will make the current `self` obsolete. It is up to you to
        /// create a new sensor that is properly configured.
        fn set_device_address(&self, address: u16) -> Result<ReplyStatus> {
            let _set = DeviceAddress(address)
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Turn off the LED.
        fn set_led_off(&self) -> Result<ReplyStatus> {
            let _set = LedOff
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Turn on the LED.
        fn set_led_on(&self) -> Result<ReplyStatus> {
            let _set = LedOn
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Get the current status of the LED.
        fn get_led_status(&self) -> Result<LedStatus> {
            let status = LedState
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(status)
        }

        /// Set the lock off for the I2C protocol mode.
        fn set_protocol_lock_off(&self) -> Result<ReplyStatus> {
            let _set = ProtocolLockDisable
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Set the lock on for the I2C protocol mode.
        fn set_protocol_lock_on(&self) -> Result<ReplyStatus> {
            let _set = ProtocolLockEnable
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Get the I2C protocol mode status.
        fn get_protocol_lock_status(&self) -> Result<ProtocolLockStatus> {
            let status = ProtocolLockState
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(status)
        }

        /// Set the sensor chip to sleep.
        ///
        /// __NOTE:__ using this command will make the sensor device sleep until:
        ///
        /// 1.  it is woken up by writing a single byte to the sensor, or
        /// 2.   by sending __any__ valid command.
        fn set_sleep(&self) -> Result<ReplyStatus> {
            let _sleep = Sleep
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }
    };

    ( reading ) => {
        /// Get the current sensor reading. Returns a `SensorReading` result.
        fn get_reading(&self) -> Result<SensorReading> {
            let reading = Reading
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(reading)
        }
    };

    ( temperature_compensation ) => {
        /// Set the compensation temperature.
        fn set_compensation(&self, value: f64) -> Result<ReplyStatus> {
            let _cmd = CompensationSet(value)
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(ReplyStatus::Ok)
        }

        /// Get the current compensated temperature value.
        fn get_compensation(&self) -> Result<CompensationValue> {
            let value = CompensationGet
                .run(&mut self.i2cdev.borrow_mut())
                .context(ErrorKind::SensorTrouble)?;
            Ok(value)
        }
    };
}

macro_rules! impl_I2CCommand_for {
    ($name:ident, $response:ty) => {
        impl I2CCommand for $name {
            type Response = $response;

            fn from_str(s: &str) -> Result<$name> {
                let cmd = s.parse::<$name>().context(ErrorKind::CommandParse)?;
                Ok(cmd)
            }

            fn to_string(&self) -> String {
                <$name as Command>::get_command_string(&self)
            }

            fn write<A, T: SensorDevice<A>>(&self, device: &T) -> Result<$response> {
                let reply = self
                    .run(&mut device.i2c_mut())
                    .context(ErrorKind::SensorTrouble)?;
                Ok(reply)
            }
        }
    };
}

macro_rules! impl_I2CResponse_for {
    ($name:ident) => {
        impl I2CResponse for $name {
            fn from_str(s: &str) -> Result<$name> {
                let response = $name::parse(s).context(ErrorKind::ResponseParse)?;
                Ok(response)
            }

            fn to_string(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
}
