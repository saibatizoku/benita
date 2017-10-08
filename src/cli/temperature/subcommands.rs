//! Temperature Subcommands
use cli::shared::{ClearSubcommand, StatusSubcommand};
use cli::shared::{
    DeviceCommand, FindCommand, LedCommand, ProtocolLockCommand, ReadCommand, SleepCommand,
    is_float,
};

use clap::{App, AppSettings, Arg, SubCommand};

/// Parses the command for Temperature sensor calibration.
pub struct TemperatureCalibrationCommand;

impl TemperatureCalibrationCommand {
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
                SubCommand::with_name("set")
                    .about("Set the calibration point command.")
                    .settings(&[AppSettings::DisableHelpSubcommand])
                    .arg(
                        Arg::with_name("TEMP")
                            .help("Numeric value up to 3 decimals.")
                            .takes_value(true)
                            .validator(is_float)
                            .required(true)
                    )
            )
    }
}

/// Parses the command for getting the Temperature sensor status.
pub type TemperatureDeviceCommand = DeviceCommand;

/// Parses the command for enabling "Find" mode on the sensor.
pub type TemperatureFindCommand = FindCommand;

/// Parses the command for setting the LED on or off on the Temperature sensor.
pub type TemperatureLedCommand = LedCommand;

/// Parses the command for setting the protocol lock on or off on the Temperature sensor.
pub type TemperatureProtocolLockCommand = ProtocolLockCommand;

/// Parses the command for taking a reading from the Temperature sensor.
pub type TemperatureReadCommand = ReadCommand;

/// Parses the command for putting the Temperature sensor to sleep (low-power mode).
pub type TemperatureSleepCommand = SleepCommand;

#[cfg(test)]
mod tests {
    use super::*;

    // Test Sleep Command.
    #[test]
    fn parsing_valid_sleep_command_input() {
        let mut cli_app = TemperatureSleepCommand::new();
        let arg_vec = vec!["sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_sleep_command_input_yields_err() {
        let mut cli_app = TemperatureSleepCommand::new();
        let arg_vec = vec!["sleep", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test Find Command.
    #[test]
    fn parsing_valid_set_find_command_input() {
        let cli_app = TemperatureFindCommand::new();
        let arg_vec = vec!["find"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_find_command_input_yields_err() {
        let mut cli_app = TemperatureFindCommand::new();
        let arg_vec = vec!["find", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test SetLedOff Command.
    #[test]
    fn parsing_valid_set_led_off_command_input() {
        let cli_app = TemperatureLedCommand::new();
        let arg_vec = vec!["led", "off"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_led_off_command_input_yields_err() {
        let mut cli_app = TemperatureLedCommand::new();
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
        let cli_app = TemperatureLedCommand::new();
        let arg_vec = vec!["led", "on"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_led_on_command_input_yields_err() {
        let mut cli_app = TemperatureLedCommand::new();
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
        let cli_app = TemperatureProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "off"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_protocol_off_command_input_yields_err() {
        let mut cli_app = TemperatureProtocolLockCommand::new();
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
        let cli_app = TemperatureProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "on"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_protocol_on_command_input_yields_err() {
        let mut cli_app = TemperatureProtocolLockCommand::new();
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
        let cli_app = TemperatureReadCommand::new();
        let arg_vec = vec!["read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_read_command_input_yields_err() {
        let mut cli_app = TemperatureReadCommand::new();
        let arg_vec = vec!["read", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GET CalibrationCommand Command.
    #[test]
    fn parsing_valid_get_calibration_command_input() {
        let cli_app = TemperatureCalibrationCommand::new();
        let arg_vec = vec!["calibration", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_calibration_command_input_yields_err() {
        let mut cli_app = TemperatureCalibrationCommand::new();
        let arg_vec = vec!["calibration"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["calibration", "status", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetDeviceInfo Command.
    #[test]
    fn parsing_valid_get_device_info_command_input() {
        let cli_app = TemperatureDeviceCommand::new();
        let arg_vec = vec!["device", "info"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_device_info_command_input_yields_err() {
        let mut cli_app = TemperatureDeviceCommand::new();
        let arg_vec = vec!["device", "info", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetLedStatus Command.
    #[test]
    fn parsing_valid_get_led_status_command_input() {
        let cli_app = TemperatureLedCommand::new();
        let arg_vec = vec!["led", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_led_status_command_input_yields_err() {
        let mut cli_app = TemperatureLedCommand::new();
        let arg_vec = vec!["led", "status", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test GetProtocolLockStatus Command.
    #[test]
    fn parsing_valid_get_protocol_status_command_input() {
        let cli_app = TemperatureProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_protocol_status_command_input_yields_err() {
        let mut cli_app = TemperatureProtocolLockCommand::new();
        let arg_vec = vec!["protocol-lock", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
        let arg_vec = vec!["protocol-lock", "status", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test TemperatureDeviceCommand.
    #[test]
    fn parsing_valid_get_device_status_command_input() {
        let cli_app = TemperatureDeviceCommand::new();
        let arg_vec = vec!["device", "status"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_get_device_status_command_input_yields_err() {
        let mut cli_app = TemperatureDeviceCommand::new();
        let arg_vec = vec!["device", "statusi"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        println!("{:?}", &matches);
        assert!(matches.is_err());
        let arg_vec = vec!["device", "status", "extra"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
