//! Networked services for pH sensing.
pub mod replies;
pub mod requests;

mod client;
mod server;

pub use self::client::PhClient;
pub use self::server::PhSensorServer;
