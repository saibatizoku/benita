//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketReply};
pub use devices::ph::responses::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus,
                                 Exported, ExportedInfo, LedStatus, ProbeSlope,
                                 ProtocolLockStatus, SensorReading};

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(CompensationValue);
impl_SocketReply_for!(DeviceInfo);
impl_SocketReply_for!(DeviceStatus);
impl_SocketReply_for!(Exported);
impl_SocketReply_for!(ExportedInfo);
impl_SocketReply_for!(LedStatus);
impl_SocketReply_for!(ProbeSlope);
impl_SocketReply_for!(ProtocolLockStatus);
impl_SocketReply_for!(SensorReading);
