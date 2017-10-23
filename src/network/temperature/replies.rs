//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketReply};
pub use devices::temperature::responses::{CalibrationStatus, DataLoggerStorageIntervalSeconds,
                                          DeviceInfo, DeviceStatus, Exported, ExportedInfo,
                                          LedStatus, MemoryReading, ProtocolLockStatus,
                                          SensorReading, TemperatureScale};

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(DataLoggerStorageIntervalSeconds);
impl_SocketReply_for!(DeviceInfo);
impl_SocketReply_for!(DeviceStatus);
impl_SocketReply_for!(Exported);
impl_SocketReply_for!(ExportedInfo);
impl_SocketReply_for!(LedStatus);
impl_SocketReply_for!(MemoryReading);
impl_SocketReply_for!(ProtocolLockStatus);
impl_SocketReply_for!(SensorReading);
impl_SocketReply_for!(TemperatureScale);
