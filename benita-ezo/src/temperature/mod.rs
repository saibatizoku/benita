//! Temperature device and network items

/// Temperature `Error`, and `ErrorKind` definitions.
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
            TemperatureDevice(device::errors::Error, device::errors::ErrorKind);
            TemperatureNetwork(network::errors::Error, network::errors::ErrorKind);

            // error chains from other crates
            EzoSensorDevice(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        }
    }
}

mod api;
pub mod command;
pub mod device;
pub mod network;
pub mod response;

pub use self::api::*;
pub use self::device::TemperatureSensor;