/// Create and define a sensor available through `i2cdev`.
#[macro_export]
macro_rules! sensor_i2cdev {
    // Name identifier and documentation for the new I2C sensor struct.
    ($name:ident , $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            path: String,
            i2cdev: LinuxI2CDevice,
        }

        impl $name {
            /// Creates a new handle for the I2C Sensor connected
            /// at the designated path and address.
            ///
            /// This device uses a file-descriptor through `i2cdev`. To use it, the path
            /// to the I2C bus, and the `u16` address location, are needed.
            pub fn new(path: &str, address: u16) -> Result<$name> {
                let i2cdev = LinuxI2CDevice::new(path, address)
                    .chain_err(|| ErrorKind::SensorTrouble)?;
                let path = path.to_string();
                Ok( $name { path, i2cdev: i2cdev } )
            }
            /// Update the sensor's I2C address.
            pub fn update_address(&mut self, address: u16) -> Result<()> {
                let i2cdev = LinuxI2CDevice::new(&self.path, address)
                    .chain_err(|| ErrorKind::SensorTrouble)?;
                self.i2cdev = i2cdev;
                Ok( () )
            }

            pub fn from_config(config: SensorConfig) -> Result<$name> {
                let config_path = match config.path.to_str() {
                    Some(path) => path,
                    _ => bail!("Invalid device path"),
                };
                $name::new(config_path, config.address)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {{ {} }}", stringify!($name), self.path)
            }
        }
    };
}
