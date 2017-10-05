//! Command-line parsers for `Conductivity` services.
use clap::{App, AppSettings, Arg, SubCommand};

/// Main command-line interface.
pub struct ConductivityApp;

impl ConductivityApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("conductivity")
            .bin_name("conductivity")
            .about("Control the conductivity sensor")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommand(ConductivityServerApp::new())
    }
}

/// Conductivity Server command-line interface .
pub struct ConductivityServerApp;

impl ConductivityServerApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("server")
            .about("REP service instance")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("rep-server-url")
                    .short("r")
                    .long("rep-server")
                    .value_name("REP_URL")
                    .help("Sets the url for the REP server")
                    .takes_value(true)
                    .index(1)
                    .required(true)
                    .conflicts_with_all(&["config"]),
            )
    }
}

/// Parses the command for temperature compensation of Conductivity readings.
pub struct ConductivitySetCompensateCommand;

impl ConductivitySetCompensateCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("compensate")
            .about("Compensation temperature used for reading calibration.")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("TEMP")
                    .help("Numeric value up to 3 decimals.")
                    .takes_value(true)
                    .index(1)
                    .required(true)
            )
    }
}

/// Parses the command for taking a reading from the Conductivity sensor.
pub struct ConductivitySetReadingCommand;

impl ConductivitySetReadingCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("sleep")
            .about("Sleep command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Parses the command for putting the Conductivity sensor to sleep (low-power mode).
pub struct ConductivitySetSleepCommand;

impl ConductivitySetSleepCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("sleep")
            .about("Sleep command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // # SET commands
    //
    // Tests for the full CLI app.
    #[test]
    fn parsing_valid_server_cli_input() {
        let cli_app = ConductivityApp::new();
        let arg_vec = vec!["conductivity", "server", "ipc://server"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        println!("matches: {:?}", &matches);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_server_cli_input_yields_err() {
        let mut cli_app = ConductivityApp::new();

        let arg_vec = vec!["server", "conductivity"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "conductivity", "server"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["conductivity", "server", "arg1", "arg2"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Tests SetCompensation Command.
    #[test]
    fn parsing_valid_compensation_command_input() {
        let mut cli_app = ConductivityCompensationCommand::new();
        let arg_vec = vec!["compensation", "set", "0.0"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_compensation_command_input_yields_err() {
        let mut cli_app = ConductivityCompensationCommand::new();
        let arg_vec = vec!["compensation"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["compensation", "set"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["compensation", "set", "non_numeric"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["compensation", "set", "1", "2"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetSleep Command.
    #[test]
    fn parsing_valid_sleep_command_input() {
        let mut cli_app = ConductivitySetSleepCommand::new();
        let arg_vec = vec!["sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_sleep_command_input_yields_err() {
        let mut cli_app = ConductivitySetSleepCommand::new();
        let arg_vec = vec!["sleep", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetFind Command.
    #[test]
    fn parsing_valid_set_find_command_input() {
        let cli_app = ConductivitySetFindCommand::new();
        let arg_vec = vec!["find"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_find_command_input_yields_err() {
        let mut cli_app = ConductivitySetFindCommand::new();
        let arg_vec = vec!["find", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetLedOff Command.
    #[test]
    fn parsing_valid_set_led_off_command_input() {
        let cli_app = ConductivityLedCommand::new();
        let arg_vec = vec!["led", "off"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_led_off_command_input_yields_err() {
        let mut cli_app = ConductivityLedCommand::new();
        let arg_vec = vec!["led"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["led", "offi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetLedOn Command.
    #[test]
    fn parsing_valid_set_led_on_command_input() {
        let cli_app = ConductivityLedCommand::new();
        let arg_vec = vec!["led", "on"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_led_on_command_input_yields_err() {
        let mut cli_app = ConductivityLedCommand::new();
        let arg_vec = vec!["led"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["led", "oni"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["led", "on", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetProtocolLockOff Command.
    #[test]
    fn parsing_valid_set_protocol_off_command_input() {
        let cli_app = ConductivityProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "off"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_protocol_off_command_input_yields_err() {
        let mut cli_app = ConductivityProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "offi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "off", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetProtocolLockOn Command.
    #[test]
    fn parsing_valid_set_protocol_on_command_input() {
        let cli_app = ConductivityProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "on"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_protocol_on_command_input_yields_err() {
        let mut cli_app = ConductivityProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "oni"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "on", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetOutputParams Command.
    #[test]
    fn parsing_valid_set_output_params_command_input() {
        let mut cli_app = ConductivitySetOutputParamsCommand::new();

        let arg_vec = vec!["output", "all"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "none"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "ec", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "ec", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "sg", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "sg", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "salinity", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "salinity", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "tds", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["output", "tds", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_output_params_command_input_yields_err() {
        let mut cli_app = ConductivitySetOutputParamsCommand::new();
        let arg_vec = vec!["output"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        println!("matches: {:?}", &matches);
        assert!(matches.is_err());
        let arg_vec = vec!["output", "wrong"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["output", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["output", "ec", "on", "tds", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // # GET commands
    //
    // Test GetReading Command.
    #[test]
    fn parsing_valid_read_command_input() {
        let cli_app = ConductivitySetReadingCommand::new();
        let arg_vec = vec!["read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_read_command_input_yields_err() {
        let mut cli_app = ConductivitySetReadingCommand::new();
        let arg_vec = vec!["read", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GET CalibrationCommand Command.
    #[test]
    fn parsing_valid_get_calibration_command_input() {
        let cli_app = ConductivityCalibrationCommand::new();
        let arg_vec = vec!["calibration", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_calibration_command_input_yields_err() {
        let mut cli_app = ConductivityCalibrationCommand::new();
        let arg_vec = vec!["calibration"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["calibration", "status", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test getting current Compensation value Command.
    #[test]
    fn parsing_valid_get_compensation_value_command_input() {
        let cli_app = ConductivityCompensationCommand::new();
        let arg_vec = vec!["compensation", "get"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_compensation_value_command_input_yields_err() {
        let mut cli_app = ConductivityCompensationCommand::new();
        let arg_vec = vec!["compensation", "get", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetDeviceInfo Command.
    #[test]
    fn parsing_valid_get_device_info_command_input() {
        let cli_app = ConductivityDeviceCommand::new();
        let arg_vec = vec!["device", "info"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_device_info_command_input_yields_err() {
        let mut cli_app = ConductivityDeviceCommand::new();
        let arg_vec = vec!["device", "info", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetLedStatus Command.
    #[test]
    fn parsing_valid_get_led_status_command_input() {
        let cli_app = ConductivityLedCommand::new();
        let arg_vec = vec!["led", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_led_status_command_input_yields_err() {
        let mut cli_app = ConductivityLedCommand::new();
        let arg_vec = vec!["led", "status", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetProbeType Command.
    #[test]
    fn parsing_valid_get_probe_type_command_input() {
        let cli_app = ConductivityProbeTypeCommand::new();
        let arg_vec = vec!["probe-type", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_probe_type_command_input_yields_err() {
        let mut cli_app = ConductivityProbeTypeCommand::new();
        let arg_vec = vec!["probe-type", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["probe-type", "status", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetProtocolLockStatus Command.
    #[test]
    fn parsing_valid_get_protocol_status_command_input() {
        let cli_app = ConductivityProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_protocol_status_command_input_yields_err() {
        let mut cli_app = ConductivityProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "status", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test ConductivityDeviceCommand.
    #[test]
    fn parsing_valid_get_device_status_command_input() {
        let cli_app = ConductivityDeviceCommand::new();
        let arg_vec = vec!["device", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_device_status_command_input_yields_err() {
        let mut cli_app = ConductivityDeviceCommand::new();
        let arg_vec = vec!["device", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        println!("{:?}", &matches);
        assert!(matches.is_err());
        let arg_vec = vec!["device", "status", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
