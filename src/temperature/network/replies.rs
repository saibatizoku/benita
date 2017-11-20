//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
pub mod errors {
    error_chain!{}
}

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
        let reply = CalibrationStatus::parse_response("?CAL,0").unwrap();
        assert_eq!("not-calibrated", &reply.to_reply_string());
        let reply = CalibrationStatus::parse_response("?CAL,1").unwrap();
        assert_eq!("calibrated", &reply.to_reply_string());
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = CalibrationStatus::parse_response("?CAL,-1");
        assert!(reply.is_err());
        let reply = CalibrationStatus::parse_response("?CAL,2");
        assert!(reply.is_err());
        let reply = CalibrationStatus::parse_response("?CAL,11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_data_logger_storage_interval_reply_from_valid_str() {
        let reply = DataLoggerStorageIntervalSeconds::parse_response("?D,0").unwrap();
        assert_eq!("0", &reply.to_reply_string());
        let reply = DataLoggerStorageIntervalSeconds::parse_response("?D,320000").unwrap();
        assert_eq!("320000", &reply.to_reply_string());
    }

    #[test]
    fn parse_data_logger_storage_interval_reply_from_invalid_str_yields_err() {
        let reply = DataLoggerStorageIntervalSeconds::parse_response("?D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_memory_reading_reply_from_valid_str() {
        let reply = MemoryReading::parse_response("10,0").unwrap();
        assert_eq!("10,0", &reply.to_reply_string());
        let reply = MemoryReading::parse_response("1,320000").unwrap();
        assert_eq!("1,320000", &reply.to_reply_string());
    }

    #[test]
    fn parse_memory_reading_reply_from_invalid_str_yields_err() {
        let reply = MemoryReading::parse_response("D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = SensorReading::parse_response("0.1").unwrap();
        assert_eq!("0.100", &reply.to_reply_string());
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = SensorReading::parse_response("");
        assert!(reply.is_err());
        let reply = SensorReading::parse_response("1.0,0.05");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_temperature_scale_reply_from_valid_str() {
        let reply = TemperatureScale::parse_response("?S,C").unwrap();
        assert_eq!("celsius", &reply.to_reply_string());
        let reply = TemperatureScale::parse_response("?S,F").unwrap();
        assert_eq!("fahrenheit", &reply.to_reply_string());
        let reply = TemperatureScale::parse_response("?S,K").unwrap();
        assert_eq!("kelvin", &reply.to_reply_string());
    }

    #[test]
    fn parse_temperature_scale_reply_from_invalid_str_yields_err() {
        let reply = TemperatureScale::parse_response("");
        assert!(reply.is_err());
    }
}
