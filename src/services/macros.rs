//! Macros for services.
macro_rules! responder_service {
    ( $doc:tt ,
      $name:ident : { $sensor:ident , $server:ident } ) => {
          #[ doc = $doc ]
          pub struct $name {
              pub server : $server,
          }

          impl $name {
              /// Create a new networked sensor service.
              pub fn new(
                  server_cfg: SocketConfig,
                  sensor_cfg: SensorConfig,
                  ) -> Result<$name> {
                      // We initialize our I2C device connection.
                      let sensor_path = match sensor_cfg.path.to_str() {
                          Some(path) => path,
                          _ => bail!("Invalid device path"),
                      };
                      let sensor = $sensor::new(sensor_path, sensor_cfg.address)
                          .chain_err(|| "Could not open I2C device")?;

                      // We configure our socket as REP, for accepting requests
                      // and providing REsPonses.
                      let responder = create_and_bind_responder(server_cfg.url)?;
                      let server = $server::new(responder, sensor)?;

                      Ok( $name { server } )
                  }
          }
    };
}

macro_rules! responder_service_process_request_functions {
    ( $cli:ident ) => {
        /// Listen for incoming command requests.
        pub fn listen(&mut self) -> Result<()> {
            loop {
                // Parse and process the command.
                let command_response: String = match self.process_request() {
                    Ok(response) => response,
                    _ => "error".to_string(),
                };
                // Send response to the client.
                let _respond = self.server.send(command_response.as_bytes())?;

                // No work left, so we sleep.
                thread::sleep(Duration::from_millis(1));
            }
        }

        /// Parse and execute incoming requests. Return an output `String`.
        pub fn process_request(&mut self) -> Result<String> {
            // Receive the incoming request
            let request_msg = self.server.recv()?;
            let cmd_args: Vec<&str> = request_msg.as_str().split(" ").collect();
            // Start the command-line interpreter
            let cli = $cli::new();
            let matched_command = cli.get_matches_from_safe(cmd_args.as_slice())
                .chain_err(|| ErrorKind::CommandParse)?;
            // Match the request subcommands to the service API.
            let response = self.run_request(&matched_command)?;
            // Return the response string.
            Ok(response)
        }

        // Process device request commands.
        fn process_device_request(&mut self, matches: &ArgMatches) -> Result<String> {
            match matches.subcommand() {
                ("address", Some(_m)) => {
                    let val = match _m.value_of("ADDRESS") {
                        Some(_val) => _val.parse::<u16>().chain_err(|| "not a number")?,
                        _ => unreachable!(),
                    };
                    self.server.set_device_address(val)
                }
                ("info", None) => self.server.get_device_info(),
                ("factory", None) => self.server.set_factory_reset(),
                ("status", None) => self.server.get_device_status(),
                _ => unreachable!(),
            }
        }

        // Process LED request commands.
        fn process_led_request(&mut self, matches: &ArgMatches) -> Result<String> {
            match matches.subcommand() {
                ("off", None) => self.server.set_led_off(),
                ("on", None) => self.server.set_led_on(),
                ("status", None) => self.server.get_led_status(),
                _ => unreachable!(),
            }
        }

        // Process protocol-lock request commands.
        fn process_protocol_lock_request(&mut self, matches: &ArgMatches) -> Result<String> {
            match matches.subcommand() {
                ("off", None) => self.server.set_protocol_lock_off(),
                ("on", None) => self.server.set_protocol_lock_on(),
                ("status", None) => self.server.get_protocol_lock_status(),
                _ => unreachable!(),
            }
        }

    };
}
