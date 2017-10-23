//! Networked services for Temperature sensing.
pub mod replies;
pub mod requests;

mod server;

pub use self::server::TemperatureSensorServer;
