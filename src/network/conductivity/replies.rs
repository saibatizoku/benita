//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketReply};
pub use devices::conductivity::responses::{CalibrationStatus, CompensationValue, DeviceInfo,
                                           DeviceStatus, Exported, ExportedInfo, LedStatus,
                                           OutputStringStatus, ProbeReading, ProbeType,
                                           ProtocolLockStatus};

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(CompensationValue);
impl_SocketReply_for!(DeviceInfo);
impl_SocketReply_for!(DeviceStatus);
impl_SocketReply_for!(Exported);
impl_SocketReply_for!(ExportedInfo);
impl_SocketReply_for!(LedStatus);
impl_SocketReply_for!(OutputStringStatus);
impl_SocketReply_for!(ProbeReading);
impl_SocketReply_for!(ProbeType);
impl_SocketReply_for!(ProtocolLockStatus);
