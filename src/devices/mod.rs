//! Collection of I2C sensor devices.

#[macro_use]
mod macros;

/// I2C device `Error`, and `ErrorKind` definitions.
pub mod errors {
    use ezo_common;
    use super::conductivity;
    use super::ph;
    use super::temperature;

    error_chain! {
        errors {
            SensorTrouble {
                description ("trouble with the sensor")
            }
        }
        links {
            Conductivity(conductivity::errors::Error, conductivity::errors::ErrorKind);
            Temperature(temperature::errors::Error, temperature::errors::ErrorKind);
            Ph(ph::errors::Error, ph::errors::ErrorKind);
            // error chains from other crates
            EzoSensor(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        }
    }
}

pub mod conductivity;
pub mod ph;
pub mod temperature;
