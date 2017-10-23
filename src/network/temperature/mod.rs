//! Networked services for Temperature sensing.
pub mod replies;

mod server;

pub use self::server::TemperatureSensorServer;
