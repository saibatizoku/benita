/// Create and define a device network socket compatible with the `benita` network.
#[macro_export]
macro_rules! network_socket {
    // Name identifier and documentation for the new network socket struct.
    ($name:ident, $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            socket: Socket,
        }

        impl $name {
            /// Create a new network socket.
            pub fn new(socket: Socket) -> Result<$name> {
                Ok($name { socket })
            }
        }

        endpoint_trait_impl!($name);
    };
}

/// Macro for declaring networked sensor sockets.
#[macro_export]
macro_rules! network_sensor_socket {
    // Simple sensor socket.
    ($name:ident, $sensor:ident, $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            socket: Socket,
            pub sensor: $sensor,
        }

        impl $name {
            /// Create a new network socket.
            pub fn new(socket: Socket, sensor: $sensor) -> Result<$name> {
                Ok($name { socket, sensor })
            }
        }

        endpoint_trait_impl!($name);
    };
}

/// Implementation of the `Endpoint` trait for a given type.
#[macro_export]
macro_rules! endpoint_trait_impl {
    ($name:ident) => {
        impl Endpoint for $name {
            /// Binds the socket to the given URL.
            fn bind(&self, url: &str) -> Result<()> {
                let _bind = self.socket.bind(url).context(ErrorKind::SocketBind)?;
                Ok(())
            }

            /// Connects the socket to the given URL.
            fn connect(&self, url: &str) -> Result<()> {
                let _connect = self.socket.connect(url).context(ErrorKind::SocketConnect)?;
                Ok(())
            }

            /// Sends a message over the network socket.
            fn send(&self, msg: &[u8]) -> Result<()> {
                let _send = self.socket.send(msg, 0).context(ErrorKind::SocketSend)?;
                Ok(())
            }

            /// Receives a message from the network socket.
            fn recv(&self) -> Result<String> {
                let received_result = self
                    .socket
                    .recv_string(0)
                    .context(ErrorKind::SocketReceive)?;
                // We match against the resulting `Resulṭ<String, Vec<u8>>`
                let response = match received_result {
                    Ok(r) => r,
                    _ => return Err(ErrorKind::ResponseParse.into()),
                };
                Ok(response)
            }
        }
    };
}

// Common network commands
macro_rules! sensor_socket_commands {
    ( calibration_common ) => {
        /// clear calibration settings.
        fn set_calibration_clear(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_calibration_clear()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }
    };
    ( calibration_status ) => {
        /// get the calibration status.
        fn get_calibration_status(&self) -> Result<CalibrationStatus> {
            let response = self.sensor
                .get_calibration_status()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }
    };

    ( device_common ) => {
        /// get the export information from the sensor.
        fn get_export_info(&self) -> Result<ExportedInfo> {
            let response = self.sensor
                .get_export_info()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// export a calibration line from the sensor.
        fn get_export_line(&self) -> Result<Exported> {
            let response = self.sensor
                .get_export_line()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// import a calibration line to the sensor.
        fn set_import_line(&self, import: &str) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_import_line(import)
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the sensor information.
        fn get_device_info(&self) -> Result<DeviceInfo> {
            let response = self.sensor
                .get_device_info()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// get the sensor status.
        fn get_device_status(&self) -> Result<DeviceStatus> {
            let response = self.sensor
                .get_device_status()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// reset the sensor device.
        fn set_factory_reset(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_factory_reset()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the sensor to find mode.
        fn set_find_mode(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_find_mode()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// change the sensor's I2C address.
        fn set_device_address(&self, address: u16) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_device_address(address)
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the LED off.
        fn set_led_off(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_led_off()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the LED on.
        fn set_led_on(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_led_on()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the current LED status.
        fn get_led_status(&self) -> Result<LedStatus> {
            let response = self.sensor
                .get_led_status()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// set the protocol lock off.
        fn set_protocol_lock_off(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_protocol_lock_off()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// set the protocol lock on.
        fn set_protocol_lock_on(&self) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_protocol_lock_on()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }

        /// get the current protocol lock status.
        fn get_protocol_lock_status(&self) -> Result<ProtocolLockStatus> {
            let response = self.sensor
                .get_protocol_lock_status()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// set the sensor to sleep (low-power) mode.
        fn set_sleep(&self) -> Result<ReplyStatus> {
            let _sleep = self.sensor
                .set_sleep()
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }
    };

    ( reading ) => {
        /// get the output string with sensor readings.
        fn get_reading(&self) -> Result<SensorReading> {
            let response = self.sensor
                .get_reading()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }
    };

    ( temperature_compensation ) => {
        /// get the compensation temperature for sensor readings.
        fn get_compensation(&self) -> Result<CompensationValue> {
            let response = self.sensor
                .get_compensation()
                .context(ErrorKind::CommandRequest)?;
            Ok(response)
        }

        /// set the compensation temperature for sensor readings.
        fn set_compensation(&self, t: f64) -> Result<ReplyStatus> {
            let _response = self.sensor
                .set_compensation(t)
                .context(ErrorKind::CommandRequest)?;
            Ok(ReplyStatus::Ok)
        }
    };
}

// Implements SocketRequest for commands
#[macro_export]
macro_rules! impl_SocketRequest_for {
    (
        $request:ident :
        $response:ident,
        $reqvalue:ident :
        $fromstr:block,
        $self:ident :
        $tostring:block
    ) => {
        impl SocketRequest for $request {
            type Response = $response;

            fn from_str(req_str: &str) -> Result<$request> {
                let $reqvalue = req_str;
                $fromstr
            }

            fn to_string(&self) -> String {
                let $self = self;
                $tostring
            }

            fn send<T: Endpoint>(&self, endpoint: &T) -> Result<$response> {
                let req = <$request as SocketRequest>::to_string(&self);
                debug!("sending socket request: {:?}", &req);
                let _read = endpoint
                    .send(req.as_bytes())
                    .context(ErrorKind::CommandRequest)?;
                let response = <$response as SocketReply>::recv(endpoint)?;
                debug!("parsed socket reply: {:?}", &response);
                Ok(response)
            }
        }
    };
}

// Macro for implementing the `SocketReply` trait on a type.
#[macro_export]
macro_rules! impl_SocketReply_for {
    ($name:ident) => {
        impl SocketReply for $name {
            fn from_str(rep_str: &str) -> Result<$name> {
                Ok($name::parse(rep_str).context(ErrorKind::CommandReply)?)
            }

            fn to_string(&self) -> String {
                format!("{}", self)
            }

            fn recv<T: Endpoint>(endpoint: &T) -> Result<$name> {
                let rep_string = endpoint.recv()?;
                debug!("received socket reply string: {:?}", &rep_string);
                let response = <$name as SocketReply>::from_str(&rep_string)?;
                debug!("parsed socket reply: {:?}", &response);
                Ok(response)
            }
        }
    };
}
