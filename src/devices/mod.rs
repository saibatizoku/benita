//! Collection of I2C sensor devices.
pub mod conductivity;
pub mod ph;
pub mod temperature;

pub mod errors {
    //! Library Error, and ErrorKind definitions.
    use ezo_common;

    error_chain! {
        errors {
            SensorTrouble {
                description ("trouble with the sensor")
            }
        }
        links {
            EzoSensor(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        }
    }
}

/// Device Errors.
pub use self::errors::*;
