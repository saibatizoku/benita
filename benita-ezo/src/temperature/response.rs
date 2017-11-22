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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calibration_status_reply_from_valid_str() {
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,0").unwrap();
        assert_eq!("?CAL,0", I2CResponse::to_string(&reply));
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,1").unwrap();
        assert_eq!("?CAL,1", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,-1");
        assert!(reply.is_err());
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,2");
        assert!(reply.is_err());
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_data_logger_storage_interval_reply_from_valid_str() {
        let reply = <DataLoggerStorageIntervalSeconds as I2CResponse>::from_str("?D,0").unwrap();
        assert_eq!("?D,0", I2CResponse::to_string(&reply));
        let reply = <DataLoggerStorageIntervalSeconds as I2CResponse>::from_str(
            "?D,320000").unwrap();
        assert_eq!("?D,320000", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_data_logger_storage_interval_reply_from_invalid_str_yields_err() {
        let reply = <DataLoggerStorageIntervalSeconds as I2CResponse>::from_str("?D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_memory_reading_reply_from_valid_str() {
        let reply = <MemoryReading as I2CResponse>::from_str("10,0").unwrap();
        assert_eq!("10,0", I2CResponse::to_string(&reply));
        let reply = <MemoryReading as I2CResponse>::from_str("1,320000").unwrap();
        assert_eq!("1,320000", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_memory_reading_reply_from_invalid_str_yields_err() {
        let reply = <MemoryReading as I2CResponse>::from_str("D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = <SensorReading as I2CResponse>::from_str("0.1").unwrap();
        assert_eq!("0.100", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = <SensorReading as I2CResponse>::from_str("");
        assert!(reply.is_err());
        let reply = <SensorReading as I2CResponse>::from_str("1.0,0.05");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_temperature_scale_reply_from_valid_str() {
        let reply = <TemperatureScale as I2CResponse>::from_str("?S,C").unwrap();
        assert_eq!("?S,C", I2CResponse::to_string(&reply));
        let reply = <TemperatureScale as I2CResponse>::from_str("?S,F").unwrap();
        assert_eq!("?S,F", I2CResponse::to_string(&reply));
        let reply = <TemperatureScale as I2CResponse>::from_str("?S,K").unwrap();
        assert_eq!("?S,K", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_temperature_scale_reply_from_invalid_str_yields_err() {
        let reply = <TemperatureScale as I2CResponse>::from_str("");
        assert!(reply.is_err());
    }
}
