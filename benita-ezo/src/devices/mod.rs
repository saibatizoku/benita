//! Collection of I2C sensor devices.

#[macro_use]
mod macros;

/// I2C device `Error`, and `ErrorKind` definitions.
pub mod errors {
    use ezo_common;

    error_chain! {
        errors {
            SensorTrouble {
                description ("trouble with the sensor")
            }
        }
        links {
            // error chains from other crates
            EzoSensorDevice(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        }
    }
}

mod traits;
pub use self::traits::*;
