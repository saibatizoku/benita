//! pH Subcommands
use cli::shared::{ClearSubcommand, StatusSubcommand};
use cli::shared::{is_float, DeviceCommand, FindCommand, LedCommand, ProtocolLockCommand,
                  ReadCommand, SleepCommand};

use clap::{App, AppSettings, Arg, SubCommand};

/// Parses the command for temperature compensation of pH readings.
pub struct PhCompensationCommand;

impl PhCompensationCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("compensation")
            .about("Compensation temperature used for reading calibration.")
            .settings(&[
                AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommand(
                SubCommand::with_name("set")
                    .about("Set compensation temperature value.")
                    .settings(&[AppSettings::DisableHelpSubcommand])
                    .arg(
                        Arg::with_name("TEMP")
                            .help("Numeric value up to 3 decimals.")
                            .takes_value(true)
                            .validator(is_float)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("get")
                    .about("Sets all parameters off.")
                    .settings(&[AppSettings::DisableHelpSubcommand]),
            )
    }
}

/// Parses the command for pH sensor calibration.
pub struct PhCalibrationCommand;

impl PhCalibrationCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("calibration")
            .about("Sensor calibration.")
            .settings(&[
                AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommand(StatusSubcommand::new())
            .subcommand(ClearSubcommand::new())
            .subcommand(
                SubCommand::with_name("high")
                    .about("Set the calibration high-point command.")
                    .settings(&[AppSettings::DisableHelpSubcommand])
                    .arg(
                        Arg::with_name("CAL")
                            .help("Numeric value up to 3 decimals.")
                            .takes_value(true)
                            .validator(is_float)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("mid")
                    .about("Set the calibration mid-point command.")
                    .settings(&[AppSettings::DisableHelpSubcommand])
                    .arg(
                        Arg::with_name("CAL")
                            .help("Numeric value up to 3 decimals.")
                            .takes_value(true)
                            .validator(is_float)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("low")
                    .about("Set the calibration low-point command.")
                    .settings(&[AppSettings::DisableHelpSubcommand])
                    .arg(
                        Arg::with_name("CAL")
                            .help("Numeric value up to 3 decimals.")
                            .takes_value(true)
                            .validator(is_float)
                            .required(true),
                    ),
            )
    }
}

/// Parses the command for getting the pH sensor status.
pub type PhDeviceCommand = DeviceCommand;

/// Parses the command for enabling "Find" mode on the sensor.
pub type PhFindCommand = FindCommand;

/// Parses the command for setting the LED on or off on the pH sensor.
pub type PhLedCommand = LedCommand;

/// Parses the command for setting the protocol lock on or off on the pH sensor.
pub type PhProtocolLockCommand = ProtocolLockCommand;

/// Parses the command for taking a reading from the pH sensor.
pub type PhReadCommand = ReadCommand;

/// Parses the command for putting the pH sensor to sleep (low-power mode).
pub type PhSleepCommand = SleepCommand;

#[cfg(test)]
mod tests {
    use super::*;

    // Tests Compensation Command.
    #[test]
    fn parsing_valid_compensation_command_input() {
        let mut cli_app = PhCompensationCommand::new();
        let arg_vec = vec!["compensation", "set", "0.0"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_compensation_command_input_yields_err() {
        let mut cli_app = PhCompensationCommand::new();
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

    // Test Sleep Command.
    #[test]
    fn parsing_valid_sleep_command_input() {
        let mut cli_app = PhSleepCommand::new();
        let arg_vec = vec!["sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_sleep_command_input_yields_err() {
        let mut cli_app = PhSleepCommand::new();
        let arg_vec = vec!["sleep", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test Find Command.
    #[test]
    fn parsing_valid_set_find_command_input() {
        let cli_app = PhFindCommand::new();
        let arg_vec = vec!["find"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_find_command_input_yields_err() {
        let mut cli_app = PhFindCommand::new();
        let arg_vec = vec!["find", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetLedOff Command.
    #[test]
    fn parsing_valid_set_led_off_command_input() {
        let cli_app = PhLedCommand::new();
        let arg_vec = vec!["led", "off"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_led_off_command_input_yields_err() {
        let mut cli_app = PhLedCommand::new();
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
        let cli_app = PhLedCommand::new();
        let arg_vec = vec!["led", "on"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_led_on_command_input_yields_err() {
        let mut cli_app = PhLedCommand::new();
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
        let cli_app = PhProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "off"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_protocol_off_command_input_yields_err() {
        let mut cli_app = PhProtocolLockCommand::new();
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
        let cli_app = PhProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "on"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_protocol_on_command_input_yields_err() {
        let mut cli_app = PhProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "oni"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "on", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // # GET commands
    //
    // Test GetReading Command.
    #[test]
    fn parsing_valid_read_command_input() {
        let cli_app = PhReadCommand::new();
        let arg_vec = vec!["read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_read_command_input_yields_err() {
        let mut cli_app = PhReadCommand::new();
        let arg_vec = vec!["read", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GET CalibrationCommand Command.
    #[test]
    fn parsing_valid_get_calibration_command_input() {
        let cli_app = PhCalibrationCommand::new();
        let arg_vec = vec!["calibration", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_calibration_command_input_yields_err() {
        let mut cli_app = PhCalibrationCommand::new();
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
        let cli_app = PhCompensationCommand::new();
        let arg_vec = vec!["compensation", "get"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_compensation_value_command_input_yields_err() {
        let mut cli_app = PhCompensationCommand::new();
        let arg_vec = vec!["compensation", "get", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetDeviceInfo Command.
    #[test]
    fn parsing_valid_get_device_info_command_input() {
        let cli_app = PhDeviceCommand::new();
        let arg_vec = vec!["device", "info"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_device_info_command_input_yields_err() {
        let mut cli_app = PhDeviceCommand::new();
        let arg_vec = vec!["device", "info", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetLedStatus Command.
    #[test]
    fn parsing_valid_get_led_status_command_input() {
        let cli_app = PhLedCommand::new();
        let arg_vec = vec!["led", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_led_status_command_input_yields_err() {
        let mut cli_app = PhLedCommand::new();
        let arg_vec = vec!["led", "status", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetProtocolLockStatus Command.
    #[test]
    fn parsing_valid_get_protocol_status_command_input() {
        let cli_app = PhProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_protocol_status_command_input_yields_err() {
        let mut cli_app = PhProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "status", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test PhDeviceCommand.
    #[test]
    fn parsing_valid_get_device_status_command_input() {
        let cli_app = PhDeviceCommand::new();
        let arg_vec = vec!["device", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_device_status_command_input_yields_err() {
        let mut cli_app = PhDeviceCommand::new();
        let arg_vec = vec!["device", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        println!("{:?}", &matches);
        assert!(matches.is_err());
        let arg_vec = vec!["device", "status", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
