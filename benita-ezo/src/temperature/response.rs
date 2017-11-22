//! Responses from EZO RTD chipset.
use errors::*;

pub use common_ezo::response::{DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus,
                               ResponseStatus, ProtocolLockStatus};
pub use ezo_rtd::response::{CalibrationStatus, DataLoggerStorageIntervalSeconds, MemoryReading,
                            SensorReading, TemperatureScale};

pub use devices::I2CResponse;


impl_I2CResponse_for!(CalibrationStatus);
impl_I2CResponse_for!(DataLoggerStorageIntervalSeconds);
impl_I2CResponse_for!(MemoryReading);
impl_I2CResponse_for!(SensorReading);
impl_I2CResponse_for!(TemperatureScale);
