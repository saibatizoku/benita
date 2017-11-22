//! Conductivity device and network items

/// Conductivity `Error`, and `ErrorKind` definitions.
pub mod errors {
    use super::device;
    use super::network;
    use ezo_common;

    error_chain! {
        errors {
            SensorTrouble {
                description ("trouble with the sensor")
            }
        }
        links {
            ConductivityDevice(device::errors::Error, device::errors::ErrorKind);
            ConductivityNetwork(network::errors::Error, network::errors::ErrorKind);
            // error chains from other crates
            EzoSensorDevice(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        }
    }
}

mod api;
pub mod device;
pub mod network;

pub mod command;
pub mod response;

pub use self::api::*;
pub use self::device::ConductivitySensor;