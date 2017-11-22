//! Responses from EZO EC chipset.
use devices::{I2CCommand, I2CResponse, SensorDevice};
use errors::*;
pub use common_ezo::response::{DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus,
                               ProtocolLockStatus, ResponseStatus};
pub use ezo_ec::response::{CalibrationStatus, CompensationValue, OutputStringStatus, ProbeType};
pub use ezo_ec::response::ProbeReading as SensorReading;

impl_I2CResponse_for!(CalibrationStatus);
impl_I2CResponse_for!(CompensationValue);
impl_I2CResponse_for!(OutputStringStatus);
impl_I2CResponse_for!(ProbeType);
impl_I2CResponse_for!(SensorReading);
