//! Requests for the conductivity sensor. Requests are sent to a conductivity `Endpoint`.
use errors::*;

use network::{Endpoint, SocketRequest, SocketResponse};
use network::common::OkResponse;

pub use devices::conductivity::commands::Baud;
pub use devices::conductivity::commands::{CalibrationClear, CalibrationDry, CalibrationHigh, CalibrationLow,
                              CalibrationOnePoint, CalibrationState};
pub use devices::conductivity::commands::{CompensationGet, CompensationSet};
pub use devices::conductivity::commands::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep,
                              Status};
pub use devices::conductivity::commands::{Export, ExportInfo, Import};
pub use devices::conductivity::commands::{LedOff, LedOn, LedState};
pub use devices::conductivity::commands::{OutputDisableConductivity, OutputDisableSalinity,
                              OutputDisableSpecificGravity, OutputDisableTds,
                              OutputEnableConductivity, OutputEnableSalinity,
                              OutputEnableSpecificGravity, OutputEnableTds, OutputState};
pub use devices::conductivity::commands::{ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen};
pub use devices::conductivity::commands::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
pub use devices::conductivity::responses::CompensationValue;
use utilities::atof;

use ezo_common::BpsRate;

// Implements SocketRequest for commands
impl SocketRequest for CompensationGet {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<CompensationGet> {
        match req_str {
            "compensation-get" => Ok(CompensationGet),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "compensation-get".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for CompensationSet {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<CompensationSet> {
        if req_str.starts_with("compensation-set ") {
            let resp = req_str.get(17..).unwrap();
            let value = atof(resp)?;
            return Ok(CompensationSet(value));
        }
        Err(ErrorKind::RequestParse.into())
    }

    fn request_string(&self) -> String {
        format!("compensation-set {:.*}", 3, self.0)
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for Reading {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<Reading> {
        match req_str {
            "read" => Ok(Reading),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "read".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for Sleep {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<Sleep> {
        match req_str {
            "sleep" => Ok(Sleep),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "sleep".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid_baud_request(test_str: &str, bps: BpsRate) {
        let request = Baud::from_request_str(test_str).unwrap();
        assert_eq!(test_str, &request.request_string());
        assert_eq!(bps, request.0);
    }
    #[test]
    fn parse_conductivity_baud_request_from_valid_str() {
        assert_valid_baud_request("baud 300", BpsRate::Bps300);
        assert_valid_baud_request("baud 1200", BpsRate::Bps1200);
        assert_valid_baud_request("baud 2400", BpsRate::Bps2400);
        assert_valid_baud_request("baud 9600", BpsRate::Bps9600);
        assert_valid_baud_request("baud 19200", BpsRate::Bps19200);
        assert_valid_baud_request("baud 38400", BpsRate::Bps38400);
        assert_valid_baud_request("baud 57600", BpsRate::Bps57600);
        assert_valid_baud_request("baud 115200", BpsRate::Bps115200);
    }

    #[test]
    fn parse_conductivity_baud_request_from_invalid_str_yields_err() {
        let request = Baud::from_request_str("baud");
        assert!(request.is_err());

        let request = Baud::from_request_str("bauds 300");
        assert!(request.is_err());

        let request = Baud::from_request_str("baud 0");
        assert!(request.is_err());

        let request = Baud::from_request_str("baud 10.5829");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_clear_request_from_valid_str() {
        let request = CalibrationClear::from_request_str("calibration-clear").unwrap();
        assert_eq!("calibration-clear", &request.request_string());
    }

    #[test]
    fn parse_conductivity_calibration_clear_request_from_invalid_str_yields_err() {
        let request = CalibrationClear::from_request_str("calibration-clearEXTRA");
        assert!(request.is_err());

        let request = CalibrationClear::from_request_str("calibration-clear 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_dry_request_from_valid_str() {
        let request = CalibrationDry::from_request_str("calibration-dry").unwrap();
        assert_eq!("calibration-dry", &request.request_string());
    }

    #[test]
    fn parse_conductivity_calibration_dry_request_from_invalid_str_yields_err() {
        let request = CalibrationDry::from_request_str("calibration-drys");
        assert!(request.is_err());

        let request = CalibrationDry::from_request_str("calibration-dry 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_high_request_from_valid_str() {
        let request = CalibrationHigh::from_request_str("calibration-high 1000.3324").unwrap();
        assert_eq!("calibration-high 1000.332", &request.request_string());
    }

    #[test]
    fn parse_conductivity_calibration_high_request_from_invalid_str_yields_err() {
        let request = CalibrationHigh::from_request_str("calibration-high");
        assert!(request.is_err());

        let request = CalibrationHigh::from_request_str("calibration-highs");
        assert!(request.is_err());

        let request = CalibrationHigh::from_request_str("calibration-high 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_low_request_from_valid_str() {
        let request = CalibrationLow::from_request_str("calibration-low 1000.3324").unwrap();
        assert_eq!("calibration-low 1000.332", &request.request_string());
    }

    #[test]
    fn parse_conductivity_calibration_low_request_from_invalid_str_yields_err() {
        let request = CalibrationLow::from_request_str("calibration-low");
        assert!(request.is_err());

        let request = CalibrationLow::from_request_str("calibration-lows");
        assert!(request.is_err());

        let request = CalibrationLow::from_request_str("calibration-low 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_onepoint_request_from_valid_str() {
        let request = CalibrationOnePoint::from_request_str("calibration-onepoint 1000.3324").unwrap();
        assert_eq!("calibration-onepoint 1000.332", &request.request_string());
    }

    #[test]
    fn parse_conductivity_calibration_onepoint_request_from_invalid_str_yields_err() {
        let request = CalibrationOnePoint::from_request_str("calibration-onepoint");
        assert!(request.is_err());

        let request = CalibrationOnePoint::from_request_str("calibration-onepoints");
        assert!(request.is_err());

        let request = CalibrationOnePoint::from_request_str("calibration-onepoint 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_status_request_from_valid_str() {
        let request = CalibrationState::from_request_str("calibration-status").unwrap();
        assert_eq!("calibration-status", &request.request_string());
    }

    #[test]
    fn parse_conductivity_calibration_status_request_from_invalid_str_yields_err() {
        let request = CalibrationState::from_request_str("calibration-statuss");
        assert!(request.is_err());

        let request = CalibrationState::from_request_str("calibration-status 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_valid_str() {
        let request = CompensationGet::from_request_str("compensation-get").unwrap();
        assert_eq!("compensation-get", &request.request_string());
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_invalid_str_yields_err() {
        let request = CompensationGet::from_request_str("ompensation-get");
        assert!(request.is_err());

        let request = CompensationGet::from_request_str("compensation-get 10.5829");
        assert!(request.is_err());

        let request = CompensationGet::from_request_str("compensation-get,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_valid_str() {
        let request = CompensationSet::from_request_str("compensation-set 10.5829").unwrap();
        assert_eq!("compensation-set 10.583", &request.request_string());
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_invalid_str_yields_err() {
        let request = CompensationSet::from_request_str("compensation-set");
        assert!(request.is_err());

        let request = CompensationSet::from_request_str("compensation-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_device_address_request_from_valid_str() {
        let request = DeviceAddress::from_request_str("device-address 90").unwrap();
        assert_eq!("device-address 90", &request.request_string());
    }

    #[test]
    fn parse_conductivity_device_address_request_from_invalid_str_yields_err() {
        let request = DeviceAddress::from_request_str("device-address");
        assert!(request.is_err());

        let request = DeviceAddress::from_request_str("device-address10.5");
        assert!(request.is_err());

        let request = DeviceAddress::from_request_str("device-address 10.5");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_device_info_request_from_valid_str() {
        let request = DeviceInformation::from_request_str("device-info").unwrap();
        assert_eq!("device-info", &request.request_string());
    }

    #[test]
    fn parse_conductivity_device_info_request_from_invalid_str_yields_err() {
        let request = DeviceInformation::from_request_str("device-infoo");
        assert!(request.is_err());

        let request = DeviceInformation::from_request_str("device-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_export_request_from_valid_str() {
        let request = Export::from_request_str("export").unwrap();
        assert_eq!("export", &request.request_string());
    }

    #[test]
    fn parse_conductivity_export_request_from_invalid_str_yields_err() {
        let request = Export::from_request_str("exporto");
        assert!(request.is_err());

        let request = Export::from_request_str("export 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_export_info_request_from_valid_str() {
        let request = ExportInfo::from_request_str("export-info").unwrap();
        assert_eq!("export-info", &request.request_string());
    }

    #[test]
    fn parse_conductivity_export_info_request_from_invalid_str_yields_err() {
        let request = ExportInfo::from_request_str("export-infoo");
        assert!(request.is_err());

        let request = ExportInfo::from_request_str("export-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_import_request_from_valid_str() {
        let request = Import::from_request_str("import 123456789012").unwrap();
        assert_eq!("import 123456789012", &request.request_string());
    }

    #[test]
    fn parse_conductivity_import_request_from_invalid_str_yields_err() {
        let request = Import::from_request_str("import");
        assert!(request.is_err());

        let request = Import::from_request_str("import ");
        assert!(request.is_err());

        let request = Import::from_request_str("import 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_factory_request_from_valid_str() {
        let request = Factory::from_request_str("factory").unwrap();
        assert_eq!("factory", &request.request_string());
    }

    #[test]
    fn parse_conductivity_factory_request_from_invalid_str_yields_err() {
        let request = Factory::from_request_str("factoryo");
        assert!(request.is_err());

        let request = Factory::from_request_str("factory 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_find_request_from_valid_str() {
        let request = Find::from_request_str("find").unwrap();
        assert_eq!("find", &request.request_string());
    }

    #[test]
    fn parse_conductivity_find_request_from_invalid_str_yields_err() {
        let request = Find::from_request_str("findo");
        assert!(request.is_err());

        let request = Find::from_request_str("find 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_led_off_request_from_valid_str() {
        let request = LedOff::from_request_str("led-off").unwrap();
        assert_eq!("led-off", &request.request_string());
    }

    #[test]
    fn parse_conductivity_led_off_request_from_invalid_str_yields_err() {
        let request = LedOff::from_request_str("led-offo");
        assert!(request.is_err());

        let request = LedOff::from_request_str("led-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_led_on_request_from_valid_str() {
        let request = LedOn::from_request_str("led-on").unwrap();
        assert_eq!("led-on", &request.request_string());
    }

    #[test]
    fn parse_conductivity_led_on_request_from_invalid_str_yields_err() {
        let request = LedOn::from_request_str("led-ono");
        assert!(request.is_err());

        let request = LedOn::from_request_str("led-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_led_status_request_from_valid_str() {
        let request = LedState::from_request_str("led-status").unwrap();
        assert_eq!("led-status", &request.request_string());
    }

    #[test]
    fn parse_conductivity_led_status_request_from_invalid_str_yields_err() {
        let request = LedState::from_request_str("led-statuso");
        assert!(request.is_err());

        let request = LedState::from_request_str("led-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_conductivity_off_request_from_valid_str() {
        let request = OutputDisableConductivity::from_request_str("output-conductivity-off").unwrap();
        assert_eq!("output-conductivity-off", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_conductivity_off_request_from_invalid_str_yields_err() {
        let request = OutputDisableConductivity::from_request_str("output-conductivity-offo");
        assert!(request.is_err());

        let request = OutputDisableConductivity::from_request_str("output-conductivity-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_salinity_off_request_from_valid_str() {
        let request = OutputDisableSalinity::from_request_str("output-salinity-off").unwrap();
        assert_eq!("output-salinity-off", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_salinity_off_request_from_invalid_str_yields_err() {
        let request = OutputDisableSalinity::from_request_str("output-salinity-offo");
        assert!(request.is_err());

        let request = OutputDisableSalinity::from_request_str("output-salinity-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_sg_off_request_from_valid_str() {
        let request = OutputDisableSpecificGravity::from_request_str("output-sg-off").unwrap();
        assert_eq!("output-sg-off", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_sg_off_request_from_invalid_str_yields_err() {
        let request = OutputDisableSpecificGravity::from_request_str("output-sg-offo");
        assert!(request.is_err());

        let request = OutputDisableSpecificGravity::from_request_str("output-sg-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_tds_off_request_from_valid_str() {
        let request = OutputDisableTds::from_request_str("output-tds-off").unwrap();
        assert_eq!("output-tds-off", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_tds_off_request_from_invalid_str_yields_err() {
        let request = OutputDisableTds::from_request_str("output-tds-offo");
        assert!(request.is_err());

        let request = OutputDisableTds::from_request_str("output-tds-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_conductivity_on_request_from_valid_str() {
        let request = OutputEnableConductivity::from_request_str("output-conductivity-on").unwrap();
        assert_eq!("output-conductivity-on", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_conductivity_on_request_from_invalid_str_yields_err() {
        let request = OutputEnableConductivity::from_request_str("output-conductivity-ono");
        assert!(request.is_err());

        let request = OutputEnableConductivity::from_request_str("output-conductivity-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_salinity_on_request_from_valid_str() {
        let request = OutputEnableSalinity::from_request_str("output-salinity-on").unwrap();
        assert_eq!("output-salinity-on", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_salinity_on_request_from_invalid_str_yields_err() {
        let request = OutputEnableSalinity::from_request_str("output-salinity-ono");
        assert!(request.is_err());

        let request = OutputEnableSalinity::from_request_str("output-salinity-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_sg_on_request_from_valid_str() {
        let request = OutputEnableSpecificGravity::from_request_str("output-sg-on").unwrap();
        assert_eq!("output-sg-on", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_sg_on_request_from_invalid_str_yields_err() {
        let request = OutputEnableSpecificGravity::from_request_str("output-sg-ono");
        assert!(request.is_err());

        let request = OutputEnableSpecificGravity::from_request_str("output-sg-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_tds_on_request_from_valid_str() {
        let request = OutputEnableTds::from_request_str("output-tds-on").unwrap();
        assert_eq!("output-tds-on", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_tds_on_request_from_invalid_str_yields_err() {
        let request = OutputEnableTds::from_request_str("output-tds-ono");
        assert!(request.is_err());

        let request = OutputEnableTds::from_request_str("output-tds-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_status_request_from_valid_str() {
        let request = OutputState::from_request_str("output-status").unwrap();
        assert_eq!("output-status", &request.request_string());
    }

    #[test]
    fn parse_conductivity_output_status_request_from_invalid_str_yields_err() {
        let request = OutputState::from_request_str("output-statuso");
        assert!(request.is_err());

        let request = OutputState::from_request_str("output-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_one_request_from_valid_str() {
        let request = ProbeTypeOne::from_request_str("probe-type-1.0").unwrap();
        assert_eq!("probe-type-1.0", &request.request_string());
    }

    #[test]
    fn parse_conductivity_probe_type_one_request_from_invalid_str_yields_err() {
        let request = ProbeTypeOne::from_request_str("probe-type-1.0 ");
        assert!(request.is_err());

        let request = ProbeTypeOne::from_request_str("probe-type-1.000000");
        assert!(request.is_err());

        let request = ProbeTypeOne::from_request_str("probe-type-1.0 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_point_one_request_from_valid_str() {
        let request = ProbeTypePointOne::from_request_str("probe-type-0.1").unwrap();
        assert_eq!("probe-type-0.1", &request.request_string());
    }

    #[test]
    fn parse_conductivity_probe_type_point_one_request_from_invalid_str_yields_err() {
        let request = ProbeTypePointOne::from_request_str("probe-type-0.1 ");
        assert!(request.is_err());

        let request = ProbeTypePointOne::from_request_str("probe-type-0.100000");
        assert!(request.is_err());

        let request = ProbeTypePointOne::from_request_str("probe-type-0.1 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_ten_request_from_valid_str() {
        let request = ProbeTypeTen::from_request_str("probe-type-10").unwrap();
        assert_eq!("probe-type-10", &request.request_string());
    }

    #[test]
    fn parse_conductivity_probe_type_ten_request_from_invalid_str_yields_err() {
        let request = ProbeTypeTen::from_request_str("probe-type-10 ");
        assert!(request.is_err());

        let request = ProbeTypeTen::from_request_str("probe-type-1000000");
        assert!(request.is_err());

        let request = ProbeTypeTen::from_request_str("probe-type-10 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_status_request_from_valid_str() {
        let request = ProbeTypeState::from_request_str("probe-type-status").unwrap();
        assert_eq!("probe-type-status", &request.request_string());
    }

    #[test]
    fn parse_conductivity_probe_type_status_request_from_invalid_str_yields_err() {
        let request = ProbeTypeState::from_request_str("probe-type-statuso");
        assert!(request.is_err());

        let request = ProbeTypeState::from_request_str("probe-type-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_protocol_lock_off_request_from_valid_str() {
        let request = ProtocolLockDisable::from_request_str("protocol-lock-off").unwrap();
        assert_eq!("protocol-lock-off", &request.request_string());
    }

    #[test]
    fn parse_conductivity_protocol_lock_off_request_from_invalid_str_yields_err() {
        let request = ProtocolLockDisable::from_request_str("protocol-lock-offo");
        assert!(request.is_err());

        let request = ProtocolLockDisable::from_request_str("protocol-lock-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_protocol_lock_on_request_from_valid_str() {
        let request = ProtocolLockEnable::from_request_str("protocol-lock-on").unwrap();
        assert_eq!("protocol-lock-on", &request.request_string());
    }

    #[test]
    fn parse_conductivity_protocol_lock_on_request_from_invalid_str_yields_err() {
        let request = ProtocolLockEnable::from_request_str("protocol-lock-ono");
        assert!(request.is_err());

        let request = ProtocolLockEnable::from_request_str("protocol-lock-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_protocol_lock_status_request_from_valid_str() {
        let request = ProtocolLockState::from_request_str("protocol-lock-status").unwrap();
        assert_eq!("protocol-lock-status", &request.request_string());
    }

    #[test]
    fn parse_conductivity_protocol_lock_status_request_from_invalid_str_yields_err() {
        let request = ProtocolLockState::from_request_str("protocol-lock-statuso");
        assert!(request.is_err());

        let request = ProtocolLockState::from_request_str("protocol-lock-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_read_request_from_valid_str() {
        let request = Reading::from_request_str("read").unwrap();
        assert_eq!("read", &request.request_string());
    }

    #[test]
    fn parse_conductivity_read_request_from_invalid_str_yields_err() {
        let request = Reading::from_request_str("reading");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_sleep_request_from_valid_str() {
        let request = Sleep::from_request_str("sleep").unwrap();
        assert_eq!("sleep", &request.request_string());
    }

    #[test]
    fn parse_conductivity_sleep_request_from_invalid_str_yields_err() {
        let request = Sleep::from_request_str("sleeping");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_status_request_from_valid_str() {
        let request = Status::from_request_str("status").unwrap();
        assert_eq!("status", &request.request_string());
    }

    #[test]
    fn parse_conductivity_status_request_from_invalid_str_yields_err() {
        let request = Status::from_request_str("statusing");
        assert!(request.is_err());
    }
}
