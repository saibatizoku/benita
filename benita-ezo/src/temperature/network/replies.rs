//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketReply};

pub use temperature::device::responses::{CalibrationStatus, DataLoggerStorageIntervalSeconds,
                                         DeviceInfo, DeviceStatus, Exported, ExportedInfo,
                                         LedStatus, MemoryReading, ProtocolLockStatus,
                                         SensorReading, TemperatureScale};

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(DataLoggerStorageIntervalSeconds);
impl_SocketReply_for!(MemoryReading);
impl_SocketReply_for!(SensorReading);
impl_SocketReply_for!(TemperatureScale);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calibration_status_reply_from_valid_str() {
        let reply = <CalibrationStatus as SocketReply>::from_str("?CAL,0").unwrap();
        assert_eq!("not-calibrated", SocketReply::to_string(&reply));
        let reply = <CalibrationStatus as SocketReply>::from_str("?CAL,1").unwrap();
        assert_eq!("calibrated", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = <CalibrationStatus as SocketReply>::from_str("?CAL,-1");
        assert!(reply.is_err());
        let reply = <CalibrationStatus as SocketReply>::from_str("?CAL,2");
        assert!(reply.is_err());
        let reply = <CalibrationStatus as SocketReply>::from_str("?CAL,11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_data_logger_storage_interval_reply_from_valid_str() {
        let reply = <DataLoggerStorageIntervalSeconds as SocketReply>::from_str("?D,0").unwrap();
        assert_eq!("0", SocketReply::to_string(&reply));
        let reply = <DataLoggerStorageIntervalSeconds as SocketReply>::from_str(
            "?D,320000").unwrap();
        assert_eq!("320000", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_data_logger_storage_interval_reply_from_invalid_str_yields_err() {
        let reply = <DataLoggerStorageIntervalSeconds as SocketReply>::from_str("?D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_memory_reading_reply_from_valid_str() {
        let reply = <MemoryReading as SocketReply>::from_str("10,0").unwrap();
        assert_eq!("10,0", SocketReply::to_string(&reply));
        let reply = <MemoryReading as SocketReply>::from_str("1,320000").unwrap();
        assert_eq!("1,320000", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_memory_reading_reply_from_invalid_str_yields_err() {
        let reply = <MemoryReading as SocketReply>::from_str("D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = <SensorReading as SocketReply>::from_str("0.1").unwrap();
        assert_eq!("0.100", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = <SensorReading as SocketReply>::from_str("");
        assert!(reply.is_err());
        let reply = <SensorReading as SocketReply>::from_str("1.0,0.05");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_temperature_scale_reply_from_valid_str() {
        let reply = <TemperatureScale as SocketReply>::from_str("?S,C").unwrap();
        assert_eq!("celsius", SocketReply::to_string(&reply));
        let reply = <TemperatureScale as SocketReply>::from_str("?S,F").unwrap();
        assert_eq!("fahrenheit", SocketReply::to_string(&reply));
        let reply = <TemperatureScale as SocketReply>::from_str("?S,K").unwrap();
        assert_eq!("kelvin", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_temperature_scale_reply_from_invalid_str_yields_err() {
        let reply = <TemperatureScale as SocketReply>::from_str("");
        assert!(reply.is_err());
    }
}
