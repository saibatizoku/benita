//! Requests for the EZO sensors. Requests are sent to an `Endpoint`, and implement the
//! `SocketRequest` trait.
use super::response::*;

use errors::*;
use network::{Endpoint, ReplyStatus, SocketReply, SocketRequest};

use ezo_common::BpsRate;

pub use super::command::*;

impl_SocketRequest_for! {
    Baud: ReplyStatus,
    req_str: {
        if req_str.starts_with("baud ") {
            let resp = req_str.get(5..).unwrap();
            let bps_num = resp.parse::<u32>()
                    .context(ErrorKind::NumberParse)?;
            let bps = BpsRate::parse_u32(bps_num)
                    .context(ErrorKind::RequestParse)?;
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
    CalibrationClear: ReplyStatus,
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
    DeviceAddress: ReplyStatus,
    req_str: {
        if req_str.starts_with("device-address ") {
            let resp = req_str.get(15..).unwrap();
            let addr = resp.parse::<u16>()
                    .context(ErrorKind::NumberParse)?;
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
    Import: ReplyStatus,
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
    Factory: ReplyStatus,
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
    Find: ReplyStatus,
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
    LedOff: ReplyStatus,
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
    LedOn: ReplyStatus,
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
    ProtocolLockDisable: ReplyStatus,
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
    ProtocolLockEnable: ReplyStatus,
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

impl_SocketRequest_for! {
    Sleep: ReplyStatus,
    req_str: {
        match req_str {
            "sleep" => Ok(Sleep),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "sleep".to_string()
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
        let request = <Baud as SocketRequest>::from_str(test_str).unwrap();
        assert_eq!(test_str, SocketRequest::to_string(&request));
        assert_eq!(bps, request.0);
    }

    #[test]
    fn parse_baud_request_from_valid_str() {
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
    fn parse_baud_request_from_invalid_str_yields_err() {
        let request = <Baud as SocketRequest>::from_str("baud");
        assert!(request.is_err());

        let request = <Baud as SocketRequest>::from_str("bauds 300");
        assert!(request.is_err());

        let request = <Baud as SocketRequest>::from_str("baud 0");
        assert!(request.is_err());

        let request = <Baud as SocketRequest>::from_str("baud 10.5829");
        assert!(request.is_err());
    }

    #[test]
    fn parse_calibration_clear_request_from_valid_str() {
        let request = <CalibrationClear as SocketRequest>::from_str("calibration-clear").unwrap();
        assert_eq!("calibration-clear", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_calibration_clear_request_from_invalid_str_yields_err() {
        let request = <CalibrationClear as SocketRequest>::from_str("calibration-clearEXTRA");
        assert!(request.is_err());

        let request = <CalibrationClear as SocketRequest>::from_str("calibration-clear 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_device_address_request_from_valid_str() {
        let request = <DeviceAddress as SocketRequest>::from_str("device-address 90").unwrap();
        assert_eq!("device-address 90", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_device_address_request_from_invalid_str_yields_err() {
        let request = <DeviceAddress as SocketRequest>::from_str("device-address");
        assert!(request.is_err());

        let request = <DeviceAddress as SocketRequest>::from_str("device-address10.5");
        assert!(request.is_err());

        let request = <DeviceAddress as SocketRequest>::from_str("device-address 10.5");
        assert!(request.is_err());
    }

    #[test]
    fn parse_device_info_request_from_valid_str() {
        let request = <DeviceInformation as SocketRequest>::from_str("device-info").unwrap();
        assert_eq!("device-info", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_device_info_request_from_invalid_str_yields_err() {
        let request = <DeviceInformation as SocketRequest>::from_str("device-infoo");
        assert!(request.is_err());

        let request = <DeviceInformation as SocketRequest>::from_str("device-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_export_request_from_valid_str() {
        let request = <Export as SocketRequest>::from_str("export").unwrap();
        assert_eq!("export", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_export_request_from_invalid_str_yields_err() {
        let request = <Export as SocketRequest>::from_str("exporto");
        assert!(request.is_err());

        let request = <Export as SocketRequest>::from_str("export 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_export_info_request_from_valid_str() {
        let request = <ExportInfo as SocketRequest>::from_str("export-info").unwrap();
        assert_eq!("export-info", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_export_info_request_from_invalid_str_yields_err() {
        let request = <ExportInfo as SocketRequest>::from_str("export-infoo");
        assert!(request.is_err());

        let request = <ExportInfo as SocketRequest>::from_str("export-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_import_request_from_valid_str() {
        let request = <Import as SocketRequest>::from_str("import 123456789012").unwrap();
        assert_eq!("import 123456789012", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_import_request_from_invalid_str_yields_err() {
        let request = <Import as SocketRequest>::from_str("import");
        assert!(request.is_err());

        let request = <Import as SocketRequest>::from_str("import ");
        assert!(request.is_err());

        let request = <Import as SocketRequest>::from_str("import 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_factory_request_from_valid_str() {
        let request = <Factory as SocketRequest>::from_str("factory").unwrap();
        assert_eq!("factory", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_factory_request_from_invalid_str_yields_err() {
        let request = <Factory as SocketRequest>::from_str("factoryo");
        assert!(request.is_err());

        let request = <Factory as SocketRequest>::from_str("factory 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_find_request_from_valid_str() {
        let request = <Find as SocketRequest>::from_str("find").unwrap();
        assert_eq!("find", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_find_request_from_invalid_str_yields_err() {
        let request = <Find as SocketRequest>::from_str("findo");
        assert!(request.is_err());

        let request = <Find as SocketRequest>::from_str("find 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_led_off_request_from_valid_str() {
        let request = <LedOff as SocketRequest>::from_str("led-off").unwrap();
        assert_eq!("led-off", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_led_off_request_from_invalid_str_yields_err() {
        let request = <LedOff as SocketRequest>::from_str("led-offo");
        assert!(request.is_err());

        let request = <LedOff as SocketRequest>::from_str("led-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_led_on_request_from_valid_str() {
        let request = <LedOn as SocketRequest>::from_str("led-on").unwrap();
        assert_eq!("led-on", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_led_on_request_from_invalid_str_yields_err() {
        let request = <LedOn as SocketRequest>::from_str("led-ono");
        assert!(request.is_err());

        let request = <LedOn as SocketRequest>::from_str("led-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_led_status_request_from_valid_str() {
        let request = <LedState as SocketRequest>::from_str("led-status").unwrap();
        assert_eq!("led-status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_led_status_request_from_invalid_str_yields_err() {
        let request = <LedState as SocketRequest>::from_str("led-statuso");
        assert!(request.is_err());

        let request = <LedState as SocketRequest>::from_str("led-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_protocol_lock_off_request_from_valid_str() {
        let request =
            <ProtocolLockDisable as SocketRequest>::from_str("protocol-lock-off").unwrap();
        assert_eq!("protocol-lock-off", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_protocol_lock_off_request_from_invalid_str_yields_err() {
        let request = <ProtocolLockDisable as SocketRequest>::from_str("protocol-lock-offo");
        assert!(request.is_err());

        let request = <ProtocolLockDisable as SocketRequest>::from_str("protocol-lock-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_protocol_lock_on_request_from_valid_str() {
        let request = <ProtocolLockEnable as SocketRequest>::from_str("protocol-lock-on").unwrap();
        assert_eq!("protocol-lock-on", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_protocol_lock_on_request_from_invalid_str_yields_err() {
        let request = <ProtocolLockEnable as SocketRequest>::from_str("protocol-lock-ono");
        assert!(request.is_err());

        let request = <ProtocolLockEnable as SocketRequest>::from_str("protocol-lock-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_protocol_lock_status_request_from_valid_str() {
        let request =
            <ProtocolLockState as SocketRequest>::from_str("protocol-lock-status").unwrap();
        assert_eq!("protocol-lock-status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_protocol_lock_status_request_from_invalid_str_yields_err() {
        let request = <ProtocolLockState as SocketRequest>::from_str("protocol-lock-statuso");
        assert!(request.is_err());

        let request = <ProtocolLockState as SocketRequest>::from_str("protocol-lock-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_sleep_request_from_valid_str() {
        let request = <Sleep as SocketRequest>::from_str("sleep").unwrap();
        assert_eq!("sleep", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_sleep_request_from_invalid_str_yields_err() {
        let request = <Sleep as SocketRequest>::from_str("sleeping");
        assert!(request.is_err());
    }

    #[test]
    fn parse_status_request_from_valid_str() {
        let request = <Status as SocketRequest>::from_str("status").unwrap();
        assert_eq!("status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_status_request_from_invalid_str_yields_err() {
        let request = <Status as SocketRequest>::from_str("statusing");
        assert!(request.is_err());
    }
}
