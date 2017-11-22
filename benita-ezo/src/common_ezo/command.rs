//! Shared commands for EZO sensors
use super::response::*;

use devices::{I2CCommand, SensorDevice};
use errors::*;

pub use ezo_common::Command;
pub use ezo_common::command::{Baud, CalibrationClear, DeviceAddress, DeviceInformation, Export,
                              ExportInfo, Factory, Find, Import, LedOff, LedOn, LedState,
                              ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState, Sleep,
                              Status};

impl_I2CCommand_for!(Baud, ResponseStatus);
impl_I2CCommand_for!(CalibrationClear, ResponseStatus);
impl_I2CCommand_for!(DeviceAddress, ResponseStatus);
impl_I2CCommand_for!(DeviceInformation, DeviceInfo);
impl_I2CCommand_for!(Export, Exported);
impl_I2CCommand_for!(ExportInfo, ExportedInfo);
impl_I2CCommand_for!(Factory, ResponseStatus);
impl_I2CCommand_for!(Find, ResponseStatus);
impl_I2CCommand_for!(Import, ResponseStatus);
impl_I2CCommand_for!(LedOff, ResponseStatus);
impl_I2CCommand_for!(LedOn, ResponseStatus);
impl_I2CCommand_for!(LedState, LedStatus);
impl_I2CCommand_for!(ProtocolLockDisable, ResponseStatus);
impl_I2CCommand_for!(ProtocolLockEnable, ResponseStatus);
impl_I2CCommand_for!(ProtocolLockState, ProtocolLockStatus);
impl_I2CCommand_for!(Sleep, ResponseStatus);
impl_I2CCommand_for!(Status, DeviceStatus);


#[cfg(test)]
mod tests {
    use super::*;
    use ezo_common::BpsRate;

    fn assert_valid_baud_request(test_str: &str, bps: BpsRate) {
        let request = <Baud as I2CCommand>::from_str(test_str).unwrap();
        assert_eq!(test_str, I2CCommand::to_string(&request));
        assert_eq!(bps, request.0);
    }

    #[test]
    fn parse_baud_request_from_valid_str() {
        assert_valid_baud_request("BAUD,300", BpsRate::Bps300);
        assert_valid_baud_request("BAUD,1200", BpsRate::Bps1200);
        assert_valid_baud_request("BAUD,2400", BpsRate::Bps2400);
        assert_valid_baud_request("BAUD,9600", BpsRate::Bps9600);
        assert_valid_baud_request("BAUD,19200", BpsRate::Bps19200);
        assert_valid_baud_request("BAUD,38400", BpsRate::Bps38400);
        assert_valid_baud_request("BAUD,57600", BpsRate::Bps57600);
        assert_valid_baud_request("BAUD,115200", BpsRate::Bps115200);
    }

    #[test]
    fn parse_baud_request_from_invalid_str_yields_err() {
        let request = <Baud as I2CCommand>::from_str("baud");
        assert!(request.is_err());

        let request = <Baud as I2CCommand>::from_str("bauds 300");
        assert!(request.is_err());

        let request = <Baud as I2CCommand>::from_str("baud 0");
        assert!(request.is_err());

        let request = <Baud as I2CCommand>::from_str("baud 10.5829");
        assert!(request.is_err());
    }

    #[test]
    fn parse_calibration_clear_request_from_valid_str() {
        let request = <CalibrationClear as I2CCommand>::from_str("cal,clear").unwrap();
        assert_eq!("CAL,CLEAR", I2CCommand::to_string(&request));
        let request = <CalibrationClear as I2CCommand>::from_str("CAL,CLEAR").unwrap();
        assert_eq!("CAL,CLEAR", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_calibration_clear_request_from_invalid_str_yields_err() {
        let request = <CalibrationClear as I2CCommand>::from_str("cal,clear,");
        assert!(request.is_err());

        let request = <CalibrationClear as I2CCommand>::from_str("cal,cleard");
        assert!(request.is_err());
    }

    #[test]
    fn parse_device_address_request_from_valid_str() {
        let request = <DeviceAddress as I2CCommand>::from_str("i2c,90").unwrap();
        assert_eq!("I2C,90", I2CCommand::to_string(&request));
        let request = <DeviceAddress as I2CCommand>::from_str("I2C,90").unwrap();
        assert_eq!("I2C,90", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_device_address_request_from_invalid_str_yields_err() {
        let request = <DeviceAddress as I2CCommand>::from_str("i2c,");
        assert!(request.is_err());

        let request = <DeviceAddress as I2CCommand>::from_str("i2c,10,");
        assert!(request.is_err());

        let request = <DeviceAddress as I2CCommand>::from_str("i2c,10.5");
        assert!(request.is_err());
    }

    #[test]
    fn parse_device_info_request_from_valid_str() {
        let request = <DeviceInformation as I2CCommand>::from_str("i").unwrap();
        assert_eq!("I", I2CCommand::to_string(&request));
        let request = <DeviceInformation as I2CCommand>::from_str("I").unwrap();
        assert_eq!("I", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_device_info_request_from_invalid_str_yields_err() {
        let request = <DeviceInformation as I2CCommand>::from_str("i,");
        assert!(request.is_err());

        let request = <DeviceInformation as I2CCommand>::from_str("info");
        assert!(request.is_err());
    }

    #[test]
    fn parse_export_request_from_valid_str() {
        let request = <Export as I2CCommand>::from_str("export").unwrap();
        assert_eq!("EXPORT", I2CCommand::to_string(&request));
        let request = <Export as I2CCommand>::from_str("EXPORT").unwrap();
        assert_eq!("EXPORT", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_export_request_from_invalid_str_yields_err() {
        let request = <Export as I2CCommand>::from_str("export,");
        assert!(request.is_err());

        let request = <Export as I2CCommand>::from_str("export,10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_export_info_request_from_valid_str() {
        let request = <ExportInfo as I2CCommand>::from_str("export,?").unwrap();
        assert_eq!("EXPORT,?", I2CCommand::to_string(&request));
        let request = <ExportInfo as I2CCommand>::from_str("EXPORT,?").unwrap();
        assert_eq!("EXPORT,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_export_info_request_from_invalid_str_yields_err() {
        let request = <ExportInfo as I2CCommand>::from_str("export,?,");
        assert!(request.is_err());

        let request = <ExportInfo as I2CCommand>::from_str("export,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_import_request_from_valid_str() {
        let request = <Import as I2CCommand>::from_str("import,123456789012").unwrap();
        assert_eq!("IMPORT,123456789012", I2CCommand::to_string(&request));
        let request = <Import as I2CCommand>::from_str("IMPORT,123456789012").unwrap();
        assert_eq!("IMPORT,123456789012", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_import_request_from_invalid_str_yields_err() {
        let request = <Import as I2CCommand>::from_str("IMPORT");
        assert!(request.is_err());

        let request = <Import as I2CCommand>::from_str("IMPORT,");
        assert!(request.is_err());

        let request = <Import as I2CCommand>::from_str("IMPORT,1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_factory_request_from_valid_str() {
        let request = <Factory as I2CCommand>::from_str("factory").unwrap();
        assert_eq!("FACTORY", I2CCommand::to_string(&request));
        let request = <Factory as I2CCommand>::from_str("FACTORY").unwrap();
        assert_eq!("FACTORY", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_factory_request_from_invalid_str_yields_err() {
        let request = <Factory as I2CCommand>::from_str("factory,");
        assert!(request.is_err());

        let request = <Factory as I2CCommand>::from_str("factory,10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_find_request_from_valid_str() {
        let request = <Find as I2CCommand>::from_str("f").unwrap();
        assert_eq!("F", I2CCommand::to_string(&request));
        let request = <Find as I2CCommand>::from_str("F").unwrap();
        assert_eq!("F", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_find_request_from_invalid_str_yields_err() {
        let request = <Find as I2CCommand>::from_str("f,");
        assert!(request.is_err());

        let request = <Find as I2CCommand>::from_str("f,10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_led_off_request_from_valid_str() {
        let request = <LedOff as I2CCommand>::from_str("l,0").unwrap();
        assert_eq!("L,0", I2CCommand::to_string(&request));
        let request = <LedOff as I2CCommand>::from_str("L,0").unwrap();
        assert_eq!("L,0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_led_off_request_from_invalid_str_yields_err() {
        let request = <LedOff as I2CCommand>::from_str("l,0,");
        assert!(request.is_err());

        let request = <LedOff as I2CCommand>::from_str("l,0a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_led_on_request_from_valid_str() {
        let request = <LedOn as I2CCommand>::from_str("l,1").unwrap();
        assert_eq!("L,1", I2CCommand::to_string(&request));
        let request = <LedOn as I2CCommand>::from_str("L,1").unwrap();
        assert_eq!("L,1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_led_on_request_from_invalid_str_yields_err() {
        let request = <LedOn as I2CCommand>::from_str("l,1,");
        assert!(request.is_err());

        let request = <LedOn as I2CCommand>::from_str("l,1a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_led_status_request_from_valid_str() {
        let request = <LedState as I2CCommand>::from_str("l,?").unwrap();
        assert_eq!("L,?", I2CCommand::to_string(&request));
        let request = <LedState as I2CCommand>::from_str("L,?").unwrap();
        assert_eq!("L,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_led_status_request_from_invalid_str_yields_err() {
        let request = <LedState as I2CCommand>::from_str("l,?,");
        assert!(request.is_err());

        let request = <LedState as I2CCommand>::from_str("l,?a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_protocol_lock_off_request_from_valid_str() {
        let request = <ProtocolLockDisable as I2CCommand>::from_str("plock,0").unwrap();
        assert_eq!("PLOCK,0", I2CCommand::to_string(&request));
        let request = <ProtocolLockDisable as I2CCommand>::from_str("PLOCK,0").unwrap();
        assert_eq!("PLOCK,0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_protocol_lock_off_request_from_invalid_str_yields_err() {
        let request = <ProtocolLockDisable as I2CCommand>::from_str("plock,0,");
        assert!(request.is_err());

        let request = <ProtocolLockDisable as I2CCommand>::from_str("plock,0a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_protocol_lock_on_request_from_valid_str() {
        let request = <ProtocolLockEnable as I2CCommand>::from_str("plock,1").unwrap();
        assert_eq!("PLOCK,1", I2CCommand::to_string(&request));
        let request = <ProtocolLockEnable as I2CCommand>::from_str("PLOCK,1").unwrap();
        assert_eq!("PLOCK,1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_protocol_lock_on_request_from_invalid_str_yields_err() {
        let request = <ProtocolLockEnable as I2CCommand>::from_str("plock,1,");
        assert!(request.is_err());

        let request = <ProtocolLockEnable as I2CCommand>::from_str("plock,1a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_protocol_lock_status_request_from_valid_str() {
        let request = <ProtocolLockState as I2CCommand>::from_str("plock,?").unwrap();
        assert_eq!("PLOCK,?", I2CCommand::to_string(&request));
        let request = <ProtocolLockState as I2CCommand>::from_str("PLOCK,?").unwrap();
        assert_eq!("PLOCK,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_protocol_lock_status_request_from_invalid_str_yields_err() {
        let request = <ProtocolLockState as I2CCommand>::from_str("plock,?,");
        assert!(request.is_err());

        let request = <ProtocolLockState as I2CCommand>::from_str("plock,?a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_sleep_request_from_valid_str() {
        let request = <Sleep as I2CCommand>::from_str("sleep").unwrap();
        assert_eq!("SLEEP", I2CCommand::to_string(&request));
        let request = <Sleep as I2CCommand>::from_str("SLEEP").unwrap();
        assert_eq!("SLEEP", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_sleep_request_from_invalid_str_yields_err() {
        let request = <Sleep as I2CCommand>::from_str("sleep,");
        assert!(request.is_err());
        let request = <Sleep as I2CCommand>::from_str("sleepy");
        assert!(request.is_err());
    }


    #[test]
    fn parse_status_request_from_valid_str() {
        let request = <Status as I2CCommand>::from_str("status").unwrap();
        assert_eq!("STATUS", I2CCommand::to_string(&request));
        let request = <Status as I2CCommand>::from_str("STATUS").unwrap();
        assert_eq!("STATUS", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_status_request_from_invalid_str_yields_err() {
        let request = <Status as I2CCommand>::from_str("statusing");
        assert!(request.is_err());
    }
}
