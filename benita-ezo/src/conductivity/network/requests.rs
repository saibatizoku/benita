//! Requests for the conductivity sensor. Requests are sent to a conductivity `Endpoint`.
pub mod errors {
    error_chain!{}
}

use conductivity::response::{CalibrationStatus, CompensationValue, OutputStringStatus, ProbeType,
                             SensorReading};

use errors::*;
use network::{Endpoint, ReplyStatus, SocketReply};
use utilities::atof;

pub use conductivity::command::{Baud, CalibrationClear, CalibrationDry, CalibrationHigh,
                                CalibrationLow, CalibrationOnePoint, CalibrationState,
                                CompensationGet, CompensationSet, DeviceAddress,
                                DeviceInformation, Export, ExportInfo, Factory, Find, Import,
                                LedOff, LedOn, LedState, OutputDisableConductivity,
                                OutputDisableSalinity, OutputDisableSpecificGravity,
                                OutputDisableTds, OutputEnableConductivity, OutputEnableSalinity,
                                OutputEnableSpecificGravity, OutputEnableTds, OutputState,
                                ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen,
                                ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState,
                                Reading, Sleep, Status};
pub use network::SocketRequest;


impl_SocketRequest_for! {
    CalibrationDry: ReplyStatus,
    req_str: {
        match req_str {
            "calibration-dry" => Ok(CalibrationDry),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "calibration-dry".to_string()
    }
}

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
    CalibrationOnePoint: ReplyStatus,
    req_str: {
        if req_str.starts_with("calibration-onepoint ") {
            let resp = req_str.get(21..).unwrap();
            let value = atof(resp)?;
            return Ok(CalibrationOnePoint(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("calibration-onepoint {:.*}", 3, req_out.0)
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
    OutputDisableConductivity: ReplyStatus,
    req_str: {
        match req_str {
            "output-conductivity-off" => Ok(OutputDisableConductivity),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-conductivity-off".to_string()
    }
}

impl_SocketRequest_for! {
    OutputDisableSalinity: ReplyStatus,
    req_str: {
        match req_str {
            "output-salinity-off" => Ok(OutputDisableSalinity),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-salinity-off".to_string()
    }
}

impl_SocketRequest_for! {
    OutputDisableSpecificGravity: ReplyStatus,
    req_str: {
        match req_str {
            "output-sg-off" => Ok(OutputDisableSpecificGravity),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-sg-off".to_string()
    }
}

impl_SocketRequest_for! {
    OutputDisableTds: ReplyStatus,
    req_str: {
        match req_str {
            "output-tds-off" => Ok(OutputDisableTds),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-tds-off".to_string()
    }
}

impl_SocketRequest_for! {
    OutputEnableConductivity: ReplyStatus,
    req_str: {
        match req_str {
            "output-conductivity-on" => Ok(OutputEnableConductivity),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-conductivity-on".to_string()
    }
}

impl_SocketRequest_for! {
    OutputEnableSalinity: ReplyStatus,
    req_str: {
        match req_str {
            "output-salinity-on" => Ok(OutputEnableSalinity),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-salinity-on".to_string()
    }
}

impl_SocketRequest_for! {
    OutputEnableSpecificGravity: ReplyStatus,
    req_str: {
        match req_str {
            "output-sg-on" => Ok(OutputEnableSpecificGravity),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-sg-on".to_string()
    }
}

impl_SocketRequest_for! {
    OutputEnableTds: ReplyStatus,
    req_str: {
        match req_str {
            "output-tds-on" => Ok(OutputEnableTds),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-tds-on".to_string()
    }
}

impl_SocketRequest_for! {
    OutputState: OutputStringStatus,
    req_str: {
        match req_str {
            "output-status" => Ok(OutputState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "output-status".to_string()
    }
}
impl_SocketRequest_for! {
    ProbeTypeOne: ReplyStatus,
    req_str: {
        match req_str {
            "probe-type-1.0" => Ok(ProbeTypeOne),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "probe-type-1.0".to_string()
    }
}

impl_SocketRequest_for! {
    ProbeTypePointOne: ReplyStatus,
    req_str: {
        match req_str {
            "probe-type-0.1" => Ok(ProbeTypePointOne),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "probe-type-0.1".to_string()
    }
}

impl_SocketRequest_for! {
    ProbeTypeTen: ReplyStatus,
    req_str: {
        match req_str {
            "probe-type-10" => Ok(ProbeTypeTen),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "probe-type-10".to_string()
    }
}

impl_SocketRequest_for! {
    ProbeTypeState: ProbeType,
    req_str: {
        match req_str {
            "probe-type-status" => Ok(ProbeTypeState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "probe-type-status".to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_conductivity_calibration_dry_request_from_valid_str() {
        let request = <CalibrationDry as SocketRequest>::from_str("calibration-dry").unwrap();
        assert_eq!("calibration-dry", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_calibration_dry_request_from_invalid_str_yields_err() {
        let request = <CalibrationDry as SocketRequest>::from_str("calibration-drys");
        assert!(request.is_err());

        let request = <CalibrationDry as SocketRequest>::from_str("calibration-dry 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_high_request_from_valid_str() {
        let request =
            <CalibrationHigh as SocketRequest>::from_str("calibration-high 1000.3324").unwrap();
        assert_eq!(
            "calibration-high 1000.332",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_calibration_high_request_from_invalid_str_yields_err() {
        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-high");
        assert!(request.is_err());

        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-highs");
        assert!(request.is_err());

        let request = <CalibrationHigh as SocketRequest>::from_str("calibration-high 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_low_request_from_valid_str() {
        let request =
            <CalibrationLow as SocketRequest>::from_str("calibration-low 1000.3324").unwrap();
        assert_eq!(
            "calibration-low 1000.332",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_calibration_low_request_from_invalid_str_yields_err() {
        let request = <CalibrationLow as SocketRequest>::from_str("calibration-low");
        assert!(request.is_err());

        let request = <CalibrationLow as SocketRequest>::from_str("calibration-lows");
        assert!(request.is_err());

        let request = <CalibrationLow as SocketRequest>::from_str("calibration-low 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_onepoint_request_from_valid_str() {
        let request = <CalibrationOnePoint as SocketRequest>::from_str(
            "calibration-onepoint 1000.3324",
        ).unwrap();
        assert_eq!(
            "calibration-onepoint 1000.332",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_calibration_onepoint_request_from_invalid_str_yields_err() {
        let request = <CalibrationOnePoint as SocketRequest>::from_str("calibration-onepoint");
        assert!(request.is_err());

        let request = <CalibrationOnePoint as SocketRequest>::from_str("calibration-onepoints");
        assert!(request.is_err());

        let request =
            <CalibrationOnePoint as SocketRequest>::from_str("calibration-onepoint 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_status_request_from_valid_str() {
        let request = <CalibrationState as SocketRequest>::from_str("calibration-status").unwrap();
        assert_eq!("calibration-status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_calibration_status_request_from_invalid_str_yields_err() {
        let request = <CalibrationState as SocketRequest>::from_str("calibration-statuss");
        assert!(request.is_err());

        let request = <CalibrationState as SocketRequest>::from_str("calibration-status 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_valid_str() {
        let request = <CompensationGet as SocketRequest>::from_str("compensation-get").unwrap();
        assert_eq!("compensation-get", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_invalid_str_yields_err() {
        let request = <CompensationGet as SocketRequest>::from_str("ompensation-get");
        assert!(request.is_err());

        let request = <CompensationGet as SocketRequest>::from_str("compensation-get 10.5829");
        assert!(request.is_err());

        let request = <CompensationGet as SocketRequest>::from_str("compensation-get,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_valid_str() {
        let request =
            <CompensationSet as SocketRequest>::from_str("compensation-set 10.5829").unwrap();
        assert_eq!(
            "compensation-set 10.583",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_invalid_str_yields_err() {
        let request = <CompensationSet as SocketRequest>::from_str("compensation-set");
        assert!(request.is_err());

        let request = <CompensationSet as SocketRequest>::from_str("compensation-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_conductivity_off_request_from_valid_str() {
        let request = <OutputDisableConductivity as SocketRequest>::from_str(
            "output-conductivity-off",
        ).unwrap();
        assert_eq!(
            "output-conductivity-off",
            SocketRequest::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_output_conductivity_off_request_from_invalid_str_yields_err() {
        let request =
            <OutputDisableConductivity as SocketRequest>::from_str("output-conductivity-offo");
        assert!(request.is_err());

        let request =
            <OutputDisableConductivity as SocketRequest>::from_str("output-conductivity-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_salinity_off_request_from_valid_str() {
        let request =
            <OutputDisableSalinity as SocketRequest>::from_str("output-salinity-off").unwrap();
        assert_eq!("output-salinity-off", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_salinity_off_request_from_invalid_str_yields_err() {
        let request = <OutputDisableSalinity as SocketRequest>::from_str("output-salinity-offo");
        assert!(request.is_err());

        let request = <OutputDisableSalinity as SocketRequest>::from_str("output-salinity-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_sg_off_request_from_valid_str() {
        let request =
            <OutputDisableSpecificGravity as SocketRequest>::from_str("output-sg-off").unwrap();
        assert_eq!("output-sg-off", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_sg_off_request_from_invalid_str_yields_err() {
        let request =
            <OutputDisableSpecificGravity as SocketRequest>::from_str("output-sg-offo");
        assert!(request.is_err());

        let request =
            <OutputDisableSpecificGravity as SocketRequest>::from_str("output-sg-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_tds_off_request_from_valid_str() {
        let request = <OutputDisableTds as SocketRequest>::from_str("output-tds-off").unwrap();
        assert_eq!("output-tds-off", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_tds_off_request_from_invalid_str_yields_err() {
        let request = <OutputDisableTds as SocketRequest>::from_str("output-tds-offo");
        assert!(request.is_err());

        let request = <OutputDisableTds as SocketRequest>::from_str("output-tds-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_conductivity_on_request_from_valid_str() {
        let request = <OutputEnableConductivity as SocketRequest>::from_str(
            "output-conductivity-on",
        ).unwrap();
        assert_eq!("output-conductivity-on", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_conductivity_on_request_from_invalid_str_yields_err() {
        let request =
            <OutputEnableConductivity as SocketRequest>::from_str("output-conductivity-ono");
        assert!(request.is_err());

        let request =
            <OutputEnableConductivity as SocketRequest>::from_str("output-conductivity-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_salinity_on_request_from_valid_str() {
        let request =
            <OutputEnableSalinity as SocketRequest>::from_str("output-salinity-on").unwrap();
        assert_eq!("output-salinity-on", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_salinity_on_request_from_invalid_str_yields_err() {
        let request = <OutputEnableSalinity as SocketRequest>::from_str("output-salinity-ono");
        assert!(request.is_err());

        let request = <OutputEnableSalinity as SocketRequest>::from_str("output-salinity-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_sg_on_request_from_valid_str() {
        let request =
            <OutputEnableSpecificGravity as SocketRequest>::from_str("output-sg-on").unwrap();
        assert_eq!("output-sg-on", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_sg_on_request_from_invalid_str_yields_err() {
        let request =
            <OutputEnableSpecificGravity as SocketRequest>::from_str("output-sg-ono");
        assert!(request.is_err());

        let request =
            <OutputEnableSpecificGravity as SocketRequest>::from_str("output-sg-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_tds_on_request_from_valid_str() {
        let request = <OutputEnableTds as SocketRequest>::from_str("output-tds-on").unwrap();
        assert_eq!("output-tds-on", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_tds_on_request_from_invalid_str_yields_err() {
        let request = <OutputEnableTds as SocketRequest>::from_str("output-tds-ono");
        assert!(request.is_err());

        let request = <OutputEnableTds as SocketRequest>::from_str("output-tds-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_status_request_from_valid_str() {
        let request = <OutputState as SocketRequest>::from_str("output-status").unwrap();
        assert_eq!("output-status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_status_request_from_invalid_str_yields_err() {
        let request = <OutputState as SocketRequest>::from_str("output-statuso");
        assert!(request.is_err());

        let request = <OutputState as SocketRequest>::from_str("output-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_one_request_from_valid_str() {
        let request = <ProbeTypeOne as SocketRequest>::from_str("probe-type-1.0").unwrap();
        assert_eq!("probe-type-1.0", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_one_request_from_invalid_str_yields_err() {
        let request = <ProbeTypeOne as SocketRequest>::from_str("probe-type-1.0 ");
        assert!(request.is_err());

        let request = <ProbeTypeOne as SocketRequest>::from_str("probe-type-1.000000");
        assert!(request.is_err());

        let request = <ProbeTypeOne as SocketRequest>::from_str("probe-type-1.0 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_point_one_request_from_valid_str() {
        let request = <ProbeTypePointOne as SocketRequest>::from_str("probe-type-0.1").unwrap();
        assert_eq!("probe-type-0.1", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_point_one_request_from_invalid_str_yields_err() {
        let request = <ProbeTypePointOne as SocketRequest>::from_str("probe-type-0.1 ");
        assert!(request.is_err());

        let request = <ProbeTypePointOne as SocketRequest>::from_str("probe-type-0.100000");
        assert!(request.is_err());

        let request =
            <ProbeTypePointOne as SocketRequest>::from_str("probe-type-0.1 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_ten_request_from_valid_str() {
        let request = <ProbeTypeTen as SocketRequest>::from_str("probe-type-10").unwrap();
        assert_eq!("probe-type-10", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_ten_request_from_invalid_str_yields_err() {
        let request = <ProbeTypeTen as SocketRequest>::from_str("probe-type-10 ");
        assert!(request.is_err());

        let request = <ProbeTypeTen as SocketRequest>::from_str("probe-type-1000000");
        assert!(request.is_err());

        let request = <ProbeTypeTen as SocketRequest>::from_str("probe-type-10 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_status_request_from_valid_str() {
        let request = <ProbeTypeState as SocketRequest>::from_str("probe-type-status").unwrap();
        assert_eq!("probe-type-status", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_status_request_from_invalid_str_yields_err() {
        let request = <ProbeTypeState as SocketRequest>::from_str("probe-type-statuso");
        assert!(request.is_err());

        let request = <ProbeTypeState as SocketRequest>::from_str("probe-type-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_read_request_from_valid_str() {
        let request = <Reading as SocketRequest>::from_str("read").unwrap();
        assert_eq!("read", SocketRequest::to_string(&request));
    }

    #[test]
    fn parse_conductivity_read_request_from_invalid_str_yields_err() {
        let request = <Reading as SocketRequest>::from_str("reading");
        assert!(request.is_err());
    }
}
