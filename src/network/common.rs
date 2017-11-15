//! Common network items.
use std;

use errors::*;

/// A response sent over a socket
pub trait Endpoint
where
    Self: std::marker::Sized,
{
    /// bind the endpoint to the given `url`. Listens for incoming messages.
    fn bind(&self, url: &str) -> Result<()>;
    /// connect the endpoint to the given `url`. Sends outgoing messages.
    fn connect(&self, url: &str) -> Result<()>;
    /// Send a slice of bytes to the endpoint.
    fn send(&self, msg: &[u8]) -> Result<()>;
    /// Receive a `String` from the endpoint.
    fn recv(&self) -> Result<String>;
}

/// A request sent over a socket
pub trait SocketRequest
where
    Self: std::marker::Sized,
{
    /// The expected response type.
    type Response: SocketReply;

    /// Create a new instance from `&str`.
    fn from_request_str(req_str: &str) -> Result<Self>;
    /// Return the instance as a `String`.
    fn to_request_string(&self) -> String;
    /// Execute the request over the socket, and return the corresponding response.
    fn send_to<T: Endpoint>(&self, &T) -> Result<Self::Response>;
}

/// A response sent over a socket
pub trait SocketReply
where
    Self: std::marker::Sized,
{
    /// Create a new instance from `&str`.
    fn parse_response(&str) -> Result<Self>;
    /// Return the instance as a `String`.
    fn to_reply_string(&self) -> String;
    /// Receive and parse the reply from the network.
    fn recv_from<T: Endpoint>(&T) -> Result<Self>;
}

// Common network commands
macro_rules! sensor_socket_commands {
    ( calibration_common ) => {
        /// clear calibration settings.
        fn set_calibration_clear(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_calibration_clear()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the calibration status.
        fn get_calibration_status(&self) -> Result<CalibrationStatus> {
            let response = self.sensor
                .get_calibration_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }
    };

    ( device_common ) => {
        /// get the export information from the sensor.
        fn get_export_info(&self) -> Result<ExportedInfo> {
            let response = self.sensor
                .get_export_info()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// export a calibration line from the sensor.
        fn get_export_line(&self) -> Result<Exported> {
            let response = self.sensor
                .get_export_line()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// import a calibration line to the sensor.
        fn set_import_line(&self, import: &str) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_import_line(import)
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the sensor information.
        fn get_device_info(&self) -> Result<DeviceInfo> {
            let response = self.sensor
                .get_device_info()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// get the sensor status.
        fn get_device_status(&self) -> Result<DeviceStatus> {
            let response = self.sensor
                .get_device_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// reset the sensor device.
        fn set_factory_reset(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_factory_reset()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the sensor to find mode.
        fn set_find_mode(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_find_mode()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// change the sensor's I2C address.
        fn set_device_address(&self, address: u16) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_device_address(address)
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the LED off.
        fn set_led_off(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_led_off()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the LED on.
        fn set_led_on(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_led_on()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the current LED status.
        fn get_led_status(&self) -> Result<LedStatus> {
            let response = self.sensor
                .get_led_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// set the protocol lock off.
        fn set_protocol_lock_off(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_protocol_lock_off()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the protocol lock on.
        fn set_protocol_lock_on(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_protocol_lock_on()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the current protocol lock status.
        fn get_protocol_lock_status(&self) -> Result<ProtocolLockStatus> {
            let response = self.sensor
                .get_protocol_lock_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// get the output string with sensor readings.
        fn get_reading(&self) -> Result<SensorReading> {
            let response = self.sensor
                .get_reading()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }
        /// set the sensor to sleep (low-power) mode.
        fn set_sleep(&self) -> Result<ReplyStatus> {
            let _sleep = self.sensor
                .set_sleep()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }
    };

    ( temperature_compensation ) => {
        /// get the compensation temperature for sensor readings.
        fn get_compensation(&self) -> Result<CompensationValue> {
            let response = self.sensor
                .get_compensation()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// set the compensation temperature for sensor readings.
        fn set_compensation(&self, t: f64) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_compensation(t)
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }
    };
}

// Implements SocketRequest for commands
macro_rules! impl_SocketRequest_for {
    (
        $request:ident : $response: ident ,
        $reqvalue:ident : $fromstr:block ,
        $self:ident : $tostring:block
    ) => {
        impl SocketRequest for $request {
            type Response = $response;

            fn from_request_str(req_str: &str) -> Result<$request> {
                let $reqvalue = req_str;
                $fromstr
            }

            fn to_request_string(&self) -> String {
                let $self = self;
                $tostring
            }

            fn send_to<T: Endpoint>(&self, endpoint: &T) -> Result<$response> {
                let req = self.to_request_string();
                debug!("sending socket request: {:?}", &req);
                let _read = endpoint.send(self.to_request_string().as_bytes())
                    .chain_err(|| ErrorKind::CommandRequest)?;
                let response = $response::recv_from(endpoint)?;
                debug!("parsed socket reply: {:?}", &response);
                Ok(response)
            }
        }
    };
}

// Macro for implementing the `SocketReply` trait on a type.
macro_rules! impl_SocketReply_for {
    ( $name:ident ) => {
        impl SocketReply for $name {
            fn parse_response(rep_str: &str) -> Result<$name> {
                $name::parse(rep_str)
                    .chain_err(|| ErrorKind::CommandReply)
            }

            fn to_reply_string(&self) -> String {
                format!("{}", self)
            }

            fn recv_from<T: Endpoint>(endpoint: &T) -> Result<$name> {
                let rep_string = endpoint.recv()?;
                debug!("received socket reply string: {:?}", &rep_string);
                let response = $name::parse_response(&rep_string)?;
                debug!("parsed socket reply: {:?}", &response);
                Ok(response)
            }
        }
    };
}

/// `ok` reply.
#[derive(PartialEq)]
pub enum ReplyStatus {
    Ok,
    Err,
}

impl ReplyStatus {
    fn parse(rep_str: &str) -> Result<ReplyStatus> {
        match rep_str {
            "ok" => Ok(ReplyStatus::Ok),
            "err" => Ok(ReplyStatus::Err),
            _ => Err(ErrorKind::ResponseParse.into()),
        }
    }
}

impl std::fmt::Debug for ReplyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReplyStatus::Ok => write!(f, "ok"),
            ReplyStatus::Err => write!(f, "err"),
        }
    }
}

impl std::fmt::Display for ReplyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_SocketReply_for!(ReplyStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_status_reply_from_valid_str() {
        let reply = ReplyStatus::parse_response("ok").unwrap();
        assert_eq!(reply, ReplyStatus::Ok);

        let reply = ReplyStatus::parse_response("err").unwrap();
        assert_eq!(reply, ReplyStatus::Err);
    }

    #[test]
    fn create_status_reply_from_invalid_str_yields_err() {
        let reply = ReplyStatus::parse_response("okerr");
        assert!(reply.is_err());
    }
}
