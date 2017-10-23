//! Common network items.
use std;
use errors::*;

/// A response sent over a socket
pub trait Endpoint where Self: std::marker::Sized {
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
pub trait SocketRequest where Self: std::marker::Sized {
    /// The expected response type.
    type Response: SocketReply;

    /// Create a new instance from `&str`.
    fn from_request_str(req_str: &str) -> Result<Self>;
    /// Return the instance as a `String`.
    fn request_string(&self) -> String;
    /// Execute the request over the socket, and return the corresponding response.
    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<Self::Response>;
}

/// A response sent over a socket
pub trait SocketReply where Self: std::marker::Sized {
    /// Create a new instance from `&str`.
    fn parse_response(rep_str: &str) -> Result<Self>;
    fn response_from<T: Endpoint>(endpoint: &T) -> Result<Self>;
}

// Implements SocketRequest for commands
pub struct OkReply;

impl SocketReply for OkReply {
    fn parse_response(rep_str: &str) -> Result<OkReply> {
        match rep_str {
            "ok" => Ok(OkReply),
            _ => Err(ErrorKind::CommandReply.into()),
        }
    }

    fn response_from<T: Endpoint>(endpoint: &T) -> Result<OkReply> {
        let rep_string = endpoint.recv()?;
        let response = OkReply::parse_response(&rep_string)?;
        Ok(response)
    }
}

// Common network commands
macro_rules! sensor_socket_commands {
    ( calibration_common ) => {
        /// clear calibration settings.
        pub fn set_calibration_clear(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_calibration_clear()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("calibration-set clear".to_string())
        }

        /// get the calibration status.
        pub fn get_calibration_status(&mut self) -> Result<String> {
            let response = self.sensor
                .get_calibration_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("calibration-status {:?}", response))
        }
    };

    ( device_common ) => {
        /// get the export information from the sensor.
        pub fn get_export_info(&mut self) -> Result<String> {
            let response = self.sensor
                .get_export_info()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("export-info {}", response))
        }

        /// export a calibration line from the sensor.
        pub fn get_export_line(&mut self) -> Result<String> {
            let response = self.sensor
                .get_export_line()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("export-line {}", response))
        }

        /// import a calibration line to the sensor.
        pub fn set_import_line(&mut self, import: &str) -> Result<String> {
            let _response = self.sensor
                .set_import_line(import)
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("imported {}", import))
        }

        /// get the sensor information.
        pub fn get_device_info(&mut self) -> Result<String> {
            let response = self.sensor
                .get_device_info()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("device-info {:?}", response))
        }

        /// get the sensor status.
        pub fn get_device_status(&mut self) -> Result<String> {
            let response = self.sensor
                .get_device_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("device-status {:?}", response))
        }

        /// reset the sensor device.
        pub fn set_factory_reset(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_factory_reset()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("device-reset".to_string())
        }

        /// set the sensor to find mode.
        pub fn set_find_mode(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_find_mode()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("device-mode find".to_string())
        }

        /// change the sensor's I2C address.
        pub fn set_device_address(&mut self, address: u16) -> Result<String> {
            let _response = self.sensor
                .set_device_address(address)
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("device-address {}", address))
        }

        /// set the LED off.
        pub fn set_led_off(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_led_off()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("device-led off".to_string())
        }

        /// set the LED on.
        pub fn set_led_on(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_led_on()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("device-led on".to_string())
        }

        /// get the current LED status.
        pub fn get_led_status(&mut self) -> Result<String> {
            let response = self.sensor
                .get_led_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("device-led-status {}", response))
        }

        /// set the protocol lock off.
        pub fn set_protocol_lock_off(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_protocol_lock_off()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("protocol-lock off".to_string())
        }

        /// set the protocol lock on.
        pub fn set_protocol_lock_on(&mut self) -> Result<String> {
            let _response = self.sensor
                .set_protocol_lock_on()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("protocol-lock on".to_string())
        }

        /// get the current protocol lock status.
        pub fn get_protocol_lock_status(&mut self) -> Result<String> {
            let response = self.sensor
                .get_protocol_lock_status()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("protocol-lock-status {}", response))
        }

        /// get the output string with sensor readings.
        pub fn get_reading(&mut self) -> Result<String> {
            let response = self.sensor
                .get_reading()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("reading {}", response))
        }
        /// set the sensor to sleep (low-power) mode.
        pub fn set_sleep(&mut self) -> Result<String> {
            let _sleep = self.sensor
                .set_sleep()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok("sleeping".to_string())
        }
    };

    ( temperature_compensation ) => {
        /// get the compensation temperature for sensor readings.
        pub fn get_compensation(&mut self) -> Result<String> {
            let response = self.sensor
                .get_compensated_temperature_value()
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("compensation-value {}", response.0))
        }

        /// set the compensation temperature for sensor readings.
        pub fn set_compensation(&mut self, t: f64) -> Result<String> {
            let _response = self.sensor
                .set_compensation_temperature(t)
                .chain_err(|| ErrorKind::CommandRequest)?;
            Ok(format!("compensation-set {}", t))
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

            fn request_string(&self) -> String {
                let $self = self;
                $tostring
            }

            fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<$response> {
                let _read = endpoint.send(self.request_string().as_bytes())
                    .chain_err(|| ErrorKind::CommandRequest)?;
                let response = $response::response_from(endpoint)?;
                Ok(response)
            }
        }
    };
}
