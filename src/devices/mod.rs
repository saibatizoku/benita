//! Collection of I2C sensor devices.
pub mod conductivity;
pub mod ph;
pub mod temperature;

mod errors {
    error_chain! {
    }
}

/// Device Errors.
pub use self::errors::*;
