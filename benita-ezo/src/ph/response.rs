//! Responses from EZO PH chipset.
use devices::I2CResponse;
use errors::*;
pub use common_ezo::response::{DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus,
                               ProtocolLockStatus, ResponseStatus};
pub use ezo_ph::response::{CalibrationStatus, CompensationValue, ProbeSlope, SensorReading};

impl_I2CResponse_for!(CalibrationStatus);
impl_I2CResponse_for!(CompensationValue);
impl_I2CResponse_for!(ProbeSlope);
impl_I2CResponse_for!(SensorReading);
