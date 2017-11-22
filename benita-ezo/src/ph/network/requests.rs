//! Requests for the pH sensor. Requests are sent to a pH `Endpoint`.
pub mod errors {
    error_chain!{}
}

use network::{Endpoint, ReplyStatus, SocketReply, SocketRequest};
use errors::*;
use ph::response::*;
use utilities::atof;

pub use common_ezo::command::*;
pub use common_ezo::response::*;
pub use ph::device::commands::*;


impl_SocketRequest_for! {
    CalibrationHigh: ReplyStatus,
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
    CalibrationLow: ReplyStatus,
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
    CalibrationMid: ReplyStatus,
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

impl_SocketRequest_for! {
    CompensationGet: CompensationValue,
    req_str: {
        match req_str {
            "compensation-get" => Ok(CompensationGet),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _reply: {
        "compensation-get".to_string()
    }
}

impl_SocketRequest_for! {
    CompensationSet: ReplyStatus,
    req_str: {
        if req_str.starts_with("compensation-set ") {
            let resp = req_str.get(17..).unwrap();
            let value = atof(resp)?;
            return Ok(CompensationSet(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    reply: {
        format!("compensation-set {:.*}", 3, reply.0)
    }
}

impl_SocketRequest_for! {
    Reading: SensorReading,
    req_str: {
        match req_str {
            "read" => Ok(Reading),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "read".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ph_calibration_high_request_from_valid_str() {
        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-high 1000.3324").unwrap();
        assert_eq!(
            "calibration-high 1000.332",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_ph_calibration_high_request_from_invalid_str_yields_err() {
        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-high");
        assert!(request.is_err());

        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-highs");
        assert!(request.is_err());

        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-high 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_low_request_from_valid_str() {
        let request = <CalibrationLow as SocketRequest>::from_str("calibration-low 1000.3324").unwrap();
        assert_eq!(
            "calibration-low 1000.332",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_ph_calibration_low_request_from_invalid_str_yields_err() {
        let request = <CalibrationLow as SocketRequest>::from_str("calibration-low");
        assert!(request.is_err());

        let request = <CalibrationLow as SocketRequest>::from_str("calibration-lows");
        assert!(request.is_err());

        let request = <CalibrationLow as SocketRequest>::from_str("calibration-low 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_mid_request_from_valid_str() {
        let request = <CalibrationMid as SocketRequest>::from_str("calibration-mid 1000.3324").unwrap();
        assert_eq!(
            "calibration-mid 1000.332",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_ph_calibration_mid_request_from_invalid_str_yields_err() {
        let request = <CalibrationMid as SocketRequest>::from_str("calibration-mid");
        assert!(request.is_err());

        let request = <CalibrationMid as SocketRequest>::from_str("calibration-mids");
        assert!(request.is_err());

        let request = <CalibrationMid as SocketRequest>::from_str("calibration-mid 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_status_request_from_valid_str() {
        let request = <CalibrationState as SocketRequest>::from_str("calibration-status").unwrap();
        assert_eq!("calibration-status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_ph_calibration_status_request_from_invalid_str_yields_err() {
        let request = <CalibrationState as SocketRequest>::from_str("calibration-statuss");
        assert!(request.is_err());

        let request = <CalibrationState as SocketRequest>::from_str("calibration-status 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_compensation_get_request_from_valid_str() {
        let request = <CompensationGet as SocketRequest>::from_str("compensation-get").unwrap();
        assert_eq!("compensation-get", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_ph_compensation_get_request_from_invalid_str_yields_err() {
        let request = <CompensationGet as SocketRequest>::from_str("ompensation-get");
        assert!(request.is_err());

        let request = <CompensationGet as SocketRequest>::from_str("compensation-get 10.5829");
        assert!(request.is_err());

        let request = <CompensationGet as SocketRequest>::from_str("compensation-get,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_compensation_set_request_from_valid_str() {
        let request = <CompensationSet as SocketRequest>::from_str("compensation-set 10.5829").unwrap();
        assert_eq!(
            "compensation-set 10.583",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_ph_compensation_set_request_from_invalid_str_yields_err() {
        let request = <CompensationSet as SocketRequest>::from_str("compensation-set");
        assert!(request.is_err());

        let request = <CompensationSet as SocketRequest>::from_str("compensation-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_read_request_from_valid_str() {
        let request = <Reading as SocketRequest>::from_str("read").unwrap();
        assert_eq!("read", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_ph_read_request_from_invalid_str_yields_err() {
        let request = <Reading as SocketRequest>::from_str("reading");
        assert!(request.is_err());
    }
    #[test]
    fn parse_ph_slope_request_from_valid_str() {
        let request = <Slope as SocketRequest>::from_str("slope").unwrap();
        assert_eq!("slope", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_ph_slope_request_from_invalid_str_yields_err() {
        let request = <Slope as SocketRequest>::from_str("slopeing");
        assert!(request.is_err());
    }
}
