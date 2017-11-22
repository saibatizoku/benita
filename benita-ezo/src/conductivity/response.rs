//! Responses from EZO EC chipset.
use devices::I2CResponse;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calibration_status_reply_from_valid_str() {
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,0").unwrap();
        assert_eq!("?CAL,0", I2CResponse::to_string(&reply));
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,1").unwrap();
        assert_eq!("?CAL,1", I2CResponse::to_string(&reply));
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,2").unwrap();
        assert_eq!("?CAL,2", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,");
        assert!(reply.is_err());
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,3");
        assert!(reply.is_err());
        let reply = <CalibrationStatus as I2CResponse>::from_str("?CAL,11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_compensation_value_reply_from_valid_str() {
        let reply = <CompensationValue as I2CResponse>::from_str("?T,0").unwrap();
        assert_eq!("?T,0.000", I2CResponse::to_string(&reply));
        let reply = <CompensationValue as I2CResponse>::from_str("?T,-15.23").unwrap();
        assert_eq!("?T,-15.230", I2CResponse::to_string(&reply));
        let reply = <CompensationValue as I2CResponse>::from_str("?T,1500.0446").unwrap();
        assert_eq!("?T,1500.045", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_compensation_value_reply_from_invalid_str_yields_err() {
        let reply = <CompensationValue as I2CResponse>::from_str("?T,");
        assert!(reply.is_err());
        let reply = <CompensationValue as I2CResponse>::from_str("?T,C11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_output_string_status_reply_from_valid_str() {
        let reply = <OutputStringStatus as I2CResponse>::from_str("?O,EC,TDS,S,SG").unwrap();
        assert_eq!("?O,EC,TDS,S,SG", I2CResponse::to_string(&reply));
        let reply = <OutputStringStatus as I2CResponse>::from_str("?O,No output").unwrap();
        assert_eq!("?O,No output", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_output_string_status_reply_from_invalid_str_yields_err() {
        let reply = <OutputStringStatus as I2CResponse>::from_str("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = <SensorReading as I2CResponse>::from_str("0.1").unwrap();
        assert_eq!("0.1", I2CResponse::to_string(&reply));
        let reply = <SensorReading as I2CResponse>::from_str("1.0,0.05").unwrap();
        assert_eq!("1,0.05", I2CResponse::to_string(&reply));
        let reply = <SensorReading as I2CResponse>::from_str("10.0").unwrap();
        assert_eq!("10", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = <SensorReading as I2CResponse>::from_str("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_type_reply_from_valid_str() {
        let reply = <ProbeType as I2CResponse>::from_str("?K,0.1").unwrap();
        assert_eq!("?K,0.1", I2CResponse::to_string(&reply));
        let reply = <ProbeType as I2CResponse>::from_str("?K,1.0").unwrap();
        assert_eq!("?K,1.0", I2CResponse::to_string(&reply));
        let reply = <ProbeType as I2CResponse>::from_str("?K,10.0").unwrap();
        assert_eq!("?K,10.0", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_probe_type_reply_from_invalid_str_yields_err() {
        let reply = <ProbeType as I2CResponse>::from_str("");
        assert!(reply.is_err());
    }
}
