//! Requests for the pH sensor. Requests are sent to a pH `Endpoint`.
use errors::*;

use network::{Endpoint, SocketReply, SocketRequest};
use network::common::OkReply;

pub use devices::ph::commands::Baud;
pub use devices::ph::commands::Command;
pub use devices::ph::commands::{CalibrationClear, CalibrationHigh, CalibrationLow, CalibrationMid,
                                CalibrationState};
pub use devices::ph::commands::{CompensationGet, CompensationSet, DeviceAddress};
pub use devices::ph::commands::{DeviceInformation, Factory, Find, Reading, Sleep, Status};
pub use devices::ph::commands::{Export, ExportInfo, Import};
pub use devices::ph::commands::{LedOff, LedOn, LedState};
pub use devices::ph::commands::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
pub use devices::ph::commands::Slope;

use devices::ph::responses::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus,
                             Exported, ExportedInfo, LedStatus, ProbeSlope, ProtocolLockStatus,
                             SensorReading};

use utilities::atof;

use ezo_common::BpsRate;

impl_SocketRequest_for! {
    Baud: OkReply,
    req_str: {
        if req_str.starts_with("baud ") {
            let resp = req_str.get(5..).unwrap();
            let bps_num = resp.parse::<u32>()
                    .chain_err(|| ErrorKind::NumberParse)?;
            let bps = BpsRate::parse_u32(bps_num)
                    .chain_err(|| ErrorKind::RequestParse)?;
            Ok(Baud(bps))
        } else {
            Err(ErrorKind::RequestParse.into())
        }
    },
    req_out: {
        format!("baud {}", &req_out.0.parse())
    }
}

impl_SocketRequest_for! {
    CalibrationClear: OkReply,
    req_str: {
        match req_str {
            "calibration-clear" => Ok(CalibrationClear),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "calibration-clear".to_string()
    }
}

impl_SocketRequest_for! {
    CalibrationHigh: OkReply,
    req_str: {
        if req_str.starts_with("calibration-high ") {
            let resp = req_str.get(17..).unwrap();
            let value = atof(resp)?;
            return Ok(CalibrationHigh(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("calibration-high {:.*}", 3, req_out.0)
    }
}

impl_SocketRequest_for! {
    CalibrationLow: OkReply,
    req_str: {
        if req_str.starts_with("calibration-low ") {
            let resp = req_str.get(16..).unwrap();
            let value = atof(resp)?;
            return Ok(CalibrationLow(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("calibration-low {:.*}", 3, req_out.0)
    }
}

impl_SocketRequest_for! {
    CalibrationMid: OkReply,
    req_str: {
        if req_str.starts_with("calibration-mid ") {
            let resp = req_str.get(16..).unwrap();
            let value = atof(resp)?;
            return Ok(CalibrationMid(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("calibration-mid {:.*}", 3, req_out.0)
    }
}

impl_SocketRequest_for! {
    CalibrationState: CalibrationStatus,
    req_str: {
        match req_str {
            "calibration-status" => Ok(CalibrationState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "calibration-status".to_string()
    }
}

impl SocketRequest for CompensationGet {
    type Response = CompensationValue;

    fn from_request_str(req_str: &str) -> Result<CompensationGet> {
        match req_str {
            "compensation-get" => Ok(CompensationGet),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "compensation-get".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<CompensationValue> {
        let _read = endpoint
            .send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = CompensationValue::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for CompensationSet {
    type Response = OkReply;

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

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<OkReply> {
        let _read = endpoint
            .send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = OkReply::response_from(endpoint)?;
        Ok(response)
    }
}

impl_SocketRequest_for! {
    DeviceAddress: OkReply,
    req_str: {
        if req_str.starts_with("device-address ") {
            let resp = req_str.get(15..).unwrap();
            let addr = resp.parse::<u16>()
                    .chain_err(|| ErrorKind::NumberParse)?;
            Ok(DeviceAddress(addr))
        } else {
            Err(ErrorKind::RequestParse.into())
        }
    },
    req_out: {
        format!("device-address {}", &req_out.0)
    }
}

impl_SocketRequest_for! {
    DeviceInformation: DeviceInfo,
    req_str: {
        match req_str {
            "device-info" => Ok(DeviceInformation),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "device-info".to_string()
    }
}

impl_SocketRequest_for! {
    Export: Exported,
    req_str: {
        match req_str {
            "export" => Ok(Export),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "export".to_string()
    }
}

impl_SocketRequest_for! {
    ExportInfo: ExportedInfo,
    req_str: {
        match req_str {
            "export-info" => Ok(ExportInfo),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "export-info".to_string()
    }
}

impl_SocketRequest_for! {
    Import: OkReply,
    req_str: {
        if req_str.starts_with("import ") {
            let resp = req_str.get(7..).unwrap();
            match resp.len() {
                1...12 => return Ok(Import(resp.to_string())),
                _ => return Err(ErrorKind::RequestParse.into()),
            }
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("import {}", req_out.0)
    }
}

impl_SocketRequest_for! {
    Factory: OkReply,
    req_str: {
        match req_str {
            "factory" => Ok(Factory),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "factory".to_string()
    }
}

impl_SocketRequest_for! {
    Find: OkReply,
    req_str: {
        match req_str {
            "find" => Ok(Find),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "find".to_string()
    }
}

impl_SocketRequest_for! {
    LedOff: OkReply,
    req_str: {
        match req_str {
            "led-off" => Ok(LedOff),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "led-off".to_string()
    }
}

impl_SocketRequest_for! {
    LedOn: OkReply,
    req_str: {
        match req_str {
            "led-on" => Ok(LedOn),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "led-on".to_string()
    }
}

impl_SocketRequest_for! {
    LedState: LedStatus,
    req_str: {
        match req_str {
            "led-status" => Ok(LedState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "led-status".to_string()
    }
}

impl_SocketRequest_for! {
    ProtocolLockDisable: OkReply,
    req_str: {
        match req_str {
            "protocol-lock-off" => Ok(ProtocolLockDisable),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "protocol-lock-off".to_string()
    }
}

impl_SocketRequest_for! {
    ProtocolLockEnable: OkReply,
    req_str: {
        match req_str {
            "protocol-lock-on" => Ok(ProtocolLockEnable),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "protocol-lock-on".to_string()
    }
}

impl_SocketRequest_for! {
    ProtocolLockState: ProtocolLockStatus,
    req_str: {
        match req_str {
            "protocol-lock-status" => Ok(ProtocolLockState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "protocol-lock-status".to_string()
    }
}

// Implements SocketRequest for commands
impl SocketRequest for Reading {
    type Response = SensorReading;

    fn from_request_str(req_str: &str) -> Result<Reading> {
        match req_str {
            "read" => Ok(Reading),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "read".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<SensorReading> {
        let _read = endpoint
            .send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = SensorReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for Sleep {
    type Response = OkReply;

    fn from_request_str(req_str: &str) -> Result<Sleep> {
        match req_str {
            "sleep" => Ok(Sleep),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "sleep".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<OkReply> {
        let _read = endpoint
            .send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = OkReply::response_from(endpoint)?;
        Ok(response)
    }
}

impl_SocketRequest_for! {
    Slope: ProbeSlope,
    req_str: {
        match req_str {
            "slope" => Ok(Slope),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "slope".to_string()
    }
}

impl_SocketRequest_for! {
    Status: DeviceStatus,
    req_str: {
        match req_str {
            "status" => Ok(Status),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "status".to_string()
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
    fn parse_ph_baud_request_from_valid_str() {
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
    fn parse_ph_baud_request_from_invalid_str_yields_err() {
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
    fn parse_ph_calibration_clear_request_from_valid_str() {
        let request = CalibrationClear::from_request_str("calibration-clear").unwrap();
        assert_eq!("calibration-clear", &request.request_string());
    }

    #[test]
    fn parse_ph_calibration_clear_request_from_invalid_str_yields_err() {
        let request = CalibrationClear::from_request_str("calibration-clearEXTRA");
        assert!(request.is_err());

        let request = CalibrationClear::from_request_str("calibration-clear 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_high_request_from_valid_str() {
        let request = CalibrationHigh::from_request_str("calibration-high 1000.3324").unwrap();
        assert_eq!("calibration-high 1000.332", &request.request_string());
    }

    #[test]
    fn parse_ph_calibration_high_request_from_invalid_str_yields_err() {
        let request = CalibrationHigh::from_request_str("calibration-high");
        assert!(request.is_err());

        let request = CalibrationHigh::from_request_str("calibration-highs");
        assert!(request.is_err());

        let request = CalibrationHigh::from_request_str("calibration-high 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_low_request_from_valid_str() {
        let request = CalibrationLow::from_request_str("calibration-low 1000.3324").unwrap();
        assert_eq!("calibration-low 1000.332", &request.request_string());
    }

    #[test]
    fn parse_ph_calibration_low_request_from_invalid_str_yields_err() {
        let request = CalibrationLow::from_request_str("calibration-low");
        assert!(request.is_err());

        let request = CalibrationLow::from_request_str("calibration-lows");
        assert!(request.is_err());

        let request = CalibrationLow::from_request_str("calibration-low 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_mid_request_from_valid_str() {
        let request = CalibrationMid::from_request_str("calibration-mid 1000.3324").unwrap();
        assert_eq!("calibration-mid 1000.332", &request.request_string());
    }

    #[test]
    fn parse_ph_calibration_mid_request_from_invalid_str_yields_err() {
        let request = CalibrationMid::from_request_str("calibration-mid");
        assert!(request.is_err());

        let request = CalibrationMid::from_request_str("calibration-mids");
        assert!(request.is_err());

        let request = CalibrationMid::from_request_str("calibration-mid 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_status_request_from_valid_str() {
        let request = CalibrationState::from_request_str("calibration-status").unwrap();
        assert_eq!("calibration-status", &request.request_string());
    }

    #[test]
    fn parse_ph_calibration_status_request_from_invalid_str_yields_err() {
        let request = CalibrationState::from_request_str("calibration-statuss");
        assert!(request.is_err());

        let request = CalibrationState::from_request_str("calibration-status 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_compensation_get_request_from_valid_str() {
        let request = CompensationGet::from_request_str("compensation-get").unwrap();
        assert_eq!("compensation-get", &request.request_string());
    }

    #[test]
    fn parse_ph_compensation_get_request_from_invalid_str_yields_err() {
        let request = CompensationGet::from_request_str("ompensation-get");
        assert!(request.is_err());

        let request = CompensationGet::from_request_str("compensation-get 10.5829");
        assert!(request.is_err());

        let request = CompensationGet::from_request_str("compensation-get,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_compensation_set_request_from_valid_str() {
        let request = CompensationSet::from_request_str("compensation-set 10.5829").unwrap();
        assert_eq!("compensation-set 10.583", &request.request_string());
    }

    #[test]
    fn parse_ph_compensation_set_request_from_invalid_str_yields_err() {
        let request = CompensationSet::from_request_str("compensation-set");
        assert!(request.is_err());

        let request = CompensationSet::from_request_str("compensation-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_device_address_request_from_valid_str() {
        let request = DeviceAddress::from_request_str("device-address 90").unwrap();
        assert_eq!("device-address 90", &request.request_string());
    }

    #[test]
    fn parse_ph_device_address_request_from_invalid_str_yields_err() {
        let request = DeviceAddress::from_request_str("device-address");
        assert!(request.is_err());

        let request = DeviceAddress::from_request_str("device-address10.5");
        assert!(request.is_err());

        let request = DeviceAddress::from_request_str("device-address 10.5");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_device_info_request_from_valid_str() {
        let request = DeviceInformation::from_request_str("device-info").unwrap();
        assert_eq!("device-info", &request.request_string());
    }

    #[test]
    fn parse_ph_device_info_request_from_invalid_str_yields_err() {
        let request = DeviceInformation::from_request_str("device-infoo");
        assert!(request.is_err());

        let request = DeviceInformation::from_request_str("device-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_export_request_from_valid_str() {
        let request = Export::from_request_str("export").unwrap();
        assert_eq!("export", &request.request_string());
    }

    #[test]
    fn parse_ph_export_request_from_invalid_str_yields_err() {
        let request = Export::from_request_str("exporto");
        assert!(request.is_err());

        let request = Export::from_request_str("export 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_export_info_request_from_valid_str() {
        let request = ExportInfo::from_request_str("export-info").unwrap();
        assert_eq!("export-info", &request.request_string());
    }

    #[test]
    fn parse_ph_export_info_request_from_invalid_str_yields_err() {
        let request = ExportInfo::from_request_str("export-infoo");
        assert!(request.is_err());

        let request = ExportInfo::from_request_str("export-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_import_request_from_valid_str() {
        let request = Import::from_request_str("import 123456789012").unwrap();
        assert_eq!("import 123456789012", &request.request_string());
    }

    #[test]
    fn parse_ph_import_request_from_invalid_str_yields_err() {
        let request = Import::from_request_str("import");
        assert!(request.is_err());

        let request = Import::from_request_str("import ");
        assert!(request.is_err());

        let request = Import::from_request_str("import 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_factory_request_from_valid_str() {
        let request = Factory::from_request_str("factory").unwrap();
        assert_eq!("factory", &request.request_string());
    }

    #[test]
    fn parse_ph_factory_request_from_invalid_str_yields_err() {
        let request = Factory::from_request_str("factoryo");
        assert!(request.is_err());

        let request = Factory::from_request_str("factory 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_find_request_from_valid_str() {
        let request = Find::from_request_str("find").unwrap();
        assert_eq!("find", &request.request_string());
    }

    #[test]
    fn parse_ph_find_request_from_invalid_str_yields_err() {
        let request = Find::from_request_str("findo");
        assert!(request.is_err());

        let request = Find::from_request_str("find 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_led_off_request_from_valid_str() {
        let request = LedOff::from_request_str("led-off").unwrap();
        assert_eq!("led-off", &request.request_string());
    }

    #[test]
    fn parse_ph_led_off_request_from_invalid_str_yields_err() {
        let request = LedOff::from_request_str("led-offo");
        assert!(request.is_err());

        let request = LedOff::from_request_str("led-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_led_on_request_from_valid_str() {
        let request = LedOn::from_request_str("led-on").unwrap();
        assert_eq!("led-on", &request.request_string());
    }

    #[test]
    fn parse_ph_led_on_request_from_invalid_str_yields_err() {
        let request = LedOn::from_request_str("led-ono");
        assert!(request.is_err());

        let request = LedOn::from_request_str("led-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_led_status_request_from_valid_str() {
        let request = LedState::from_request_str("led-status").unwrap();
        assert_eq!("led-status", &request.request_string());
    }

    #[test]
    fn parse_ph_led_status_request_from_invalid_str_yields_err() {
        let request = LedState::from_request_str("led-statuso");
        assert!(request.is_err());

        let request = LedState::from_request_str("led-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_protocol_lock_off_request_from_valid_str() {
        let request = ProtocolLockDisable::from_request_str("protocol-lock-off").unwrap();
        assert_eq!("protocol-lock-off", &request.request_string());
    }

    #[test]
    fn parse_ph_protocol_lock_off_request_from_invalid_str_yields_err() {
        let request = ProtocolLockDisable::from_request_str("protocol-lock-offo");
        assert!(request.is_err());

        let request = ProtocolLockDisable::from_request_str("protocol-lock-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_protocol_lock_on_request_from_valid_str() {
        let request = ProtocolLockEnable::from_request_str("protocol-lock-on").unwrap();
        assert_eq!("protocol-lock-on", &request.request_string());
    }

    #[test]
    fn parse_ph_protocol_lock_on_request_from_invalid_str_yields_err() {
        let request = ProtocolLockEnable::from_request_str("protocol-lock-ono");
        assert!(request.is_err());

        let request = ProtocolLockEnable::from_request_str("protocol-lock-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_protocol_lock_status_request_from_valid_str() {
        let request = ProtocolLockState::from_request_str("protocol-lock-status").unwrap();
        assert_eq!("protocol-lock-status", &request.request_string());
    }

    #[test]
    fn parse_ph_protocol_lock_status_request_from_invalid_str_yields_err() {
        let request = ProtocolLockState::from_request_str("protocol-lock-statuso");
        assert!(request.is_err());

        let request = ProtocolLockState::from_request_str("protocol-lock-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_read_request_from_valid_str() {
        let request = Reading::from_request_str("read").unwrap();
        assert_eq!("read", &request.request_string());
    }

    #[test]
    fn parse_ph_read_request_from_invalid_str_yields_err() {
        let request = Reading::from_request_str("reading");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_sleep_request_from_valid_str() {
        let request = Sleep::from_request_str("sleep").unwrap();
        assert_eq!("sleep", &request.request_string());
    }

    #[test]
    fn parse_ph_sleep_request_from_invalid_str_yields_err() {
        let request = Sleep::from_request_str("sleeping");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_slope_request_from_valid_str() {
        let request = Slope::from_request_str("slope").unwrap();
        assert_eq!("slope", &request.request_string());
    }

    #[test]
    fn parse_ph_slope_request_from_invalid_str_yields_err() {
        let request = Slope::from_request_str("slopeing");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_status_request_from_valid_str() {
        let request = Status::from_request_str("status").unwrap();
        assert_eq!("status", &request.request_string());
    }

    #[test]
    fn parse_ph_status_request_from_invalid_str_yields_err() {
        let request = Status::from_request_str("statusing");
        assert!(request.is_err());
    }
}
