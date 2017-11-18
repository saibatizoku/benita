//! Conductivity device and network items

/// Conductivity `Error`, and `ErrorKind` definitions.
pub mod errors {
    use super::device;
    use ezo_common;

    error_chain! {
        errors {
            SensorTrouble {
                description ("trouble with the sensor")
            }
        }
        links {
            ConductivityDevice(device::errors::Error, device::errors::ErrorKind);
            // error chains from other crates
            EzoSensorDevice(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        }
    }
}

mod api;
pub mod device;

pub use self::api::*;
pub use self::device::ConductivitySensor;
