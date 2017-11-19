//! Responses from EZO EC chipset.
pub use ezo_ec::response::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus,
                           Exported, ExportedInfo, LedStatus, OutputStringStatus,
                           ProbeType, ProtocolLockStatus};
pub use ezo_ec::response::ProbeReading as SensorReading;
