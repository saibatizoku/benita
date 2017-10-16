//! Conductivity Subcommands
use cli::shared::{ClearSubcommand, GetSubcommand, OffSubcommand, OnSubcommand, SetSubcommand,
                  StatusSubcommand};
use cli::shared::{is_float, DeviceCommand, FindCommand, LedCommand, ProtocolLockCommand,
                  ReadCommand, SleepCommand};

use clap::{App, AppSettings, Arg, SubCommand};


/// Top-level `conductivity` command.
pub struct ConductivityCompensationCommand;

impl ConductivityCompensationCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("compensation")
            .about("Compensation temperature used for reading calibration.")
            .settings(&[
                AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommand(SetSubcommand::new())
            .subcommand(GetSubcommand::new())
    }
}

/// Parses the command for configuring the output string on the Conductivity sensor.
pub struct ConductivityOutputParamsCommand;

impl ConductivityOutputParamsCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("output")
            .about("Set the parameters printed in the Output string.")
            .settings(&[
                AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommand(
                SubCommand::with_name("ec")
                    .about("Enables/disables the electric-conductivity in the output string.")
                    .settings(&[
                        AppSettings::DisableHelpSubcommand,
                        AppSettings::SubcommandRequired,
                    ])
                    .subcommand(OffSubcommand::new())
                    .subcommand(OnSubcommand::new()),
            )
            .subcommand(
                SubCommand::with_name("salinity")
                    .about("Enables/disables the salinity in the output string.")
                    .settings(&[
                        AppSettings::DisableHelpSubcommand,
                        AppSettings::SubcommandRequired,
                    ])
                    .subcommand(OffSubcommand::new())
                    .subcommand(OnSubcommand::new()),
            )
            .subcommand(
                SubCommand::with_name("sg")
                    .about("Enables/disables the specific-gravity in the output string.")
                    .settings(&[
                        AppSettings::DisableHelpSubcommand,
                        AppSettings::SubcommandRequired,
                    ])
                    .subcommand(OffSubcommand::new())
                    .subcommand(OnSubcommand::new()),
            )
            .subcommand(
                SubCommand::with_name("tds")
                    .about("Enables/disables the total-dissolved solids in the output string.")
                    .settings(&[
                        AppSettings::DisableHelpSubcommand,
                        AppSettings::SubcommandRequired,
                    ])
                    .subcommand(OffSubcommand::new())
                    .subcommand(OnSubcommand::new()),
            )
            .subcommand(StatusSubcommand::new())
    }
}

/// Parses the command for getting the Conductivity sensor information.
pub struct ConductivityProbeTypeCommand;

impl ConductivityProbeTypeCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("probe-type")
            .about("Probe-type command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("probe")
                    .help("Sets/gets the sensor's probe type.")
                    .takes_value(true)
                    .possible_values(&["status", "0.1", "1.0", "10.0"])
                    .required(true),
            )
    }
}

/// Parses the command for Conductivity sensor calibration.
pub struct ConductivityCalibrationCommand;

impl ConductivityCalibrationCommand {
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
                SubCommand::with_name("dry")
                    .about("Set the calibration dry-point command.")
                    .settings(&[AppSettings::DisableHelpSubcommand]),
            )
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
            .subcommand(
                SubCommand::with_name("single")
                    .about("Set the calibration single-point command.")
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

/// Parses the command for getting the Conductivity sensor status.
pub type ConductivityDeviceCommand = DeviceCommand;

/// Parses the command for enabling "Find" mode on the sensor.
pub type ConductivityFindCommand = FindCommand;

/// Parses the command for setting the LED on or off on the Conductivity sensor.
pub type ConductivityLedCommand = LedCommand;

/// Parses the command for setting the protocol lock on or off on the Conductivity sensor.
pub type ConductivityProtocolLockCommand = ProtocolLockCommand;

/// Parses the command for taking a reading from the Conductivity sensor.
pub type ConductivityReadCommand = ReadCommand;

/// Parses the command for putting the Conductivity sensor to sleep (low-power mode).
pub type ConductivitySleepCommand = SleepCommand;

#[cfg(test)]
mod tests {
    use super::*;

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

    // Test Sleep Command.
    #[test]
    fn parsing_valid_sleep_command_input() {
        let mut cli_app = ConductivitySleepCommand::new();
        let arg_vec = vec!["sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_sleep_command_input_yields_err() {
        let mut cli_app = ConductivitySleepCommand::new();
        let arg_vec = vec!["sleep", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Test Find Command.
    #[test]
    fn parsing_valid_set_find_command_input() {
        let cli_app = ConductivityFindCommand::new();
        let arg_vec = vec!["find"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_set_find_command_input_yields_err() {
        let mut cli_app = ConductivityFindCommand::new();
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

    // Test OutputParams Command.
    #[test]
    fn parsing_valid_set_output_params_command_input() {
        let mut cli_app = ConductivityOutputParamsCommand::new();

        let arg_vec = vec!["output", "status"];
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
        let mut cli_app = ConductivityOutputParamsCommand::new();
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
        let cli_app = ConductivityReadCommand::new();
        let arg_vec = vec!["read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_read_command_input_yields_err() {
        let mut cli_app = ConductivityReadCommand::new();
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
