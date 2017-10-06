//! Shared subcommands and utility functions.
use clap::{App, AppSettings, Arg, SubCommand};
use neuras::url::Url;

/// Validator function for URLs.
pub fn is_url(v: String) -> Result<(), String> {
    match Url::parse(&v) {
        Ok(_) => Ok(()),
        _ => Err("Invalid URL".to_string()),
    }
}

/// Validator function for floating point numbers.
pub fn is_float(v: String) -> Result<(), String> {
    match v.parse::<f64>() {
        Ok(_) => Ok(()),
        _ => Err("The value is not numeric.".to_string()),
    }
}

/// Set a parameter `off`.
pub struct OffSubcommand;

impl OffSubcommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("off")
            .about("Sets parameter off.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Set a parameter `on`.
pub struct OnSubcommand;

impl OnSubcommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("on")
            .about("Sets parameter on.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Get the current parameter status.
pub struct StatusSubcommand;

impl StatusSubcommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("status")
            .about("Gets parameter status.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Parses the command for getting the sensor status.
pub struct DeviceCommand;

impl DeviceCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("device")
            .about("Device status/information command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("param")
                    .help("Get device status or information.")
                    .takes_value(true)
                    .possible_values(&["status", "info"])
                    .required(true)
            )
    }
}

/// Parses the command for enabling "Find" mode on the sensor.
pub struct FindCommand;

impl FindCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("find")
            .about("Set the sensor in FIND mode.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Parses the command for setting the LED on or off on the sensor.
pub struct LedCommand;

impl LedCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("led")
            .about("LED on|off|status command.")
            .settings(&[
                AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommand(OffSubcommand::new())
            .subcommand(OnSubcommand::new())
            .subcommand(StatusSubcommand::new())
    }
}

/// Parses the command for setting the protocol lock on or off on the sensor.
pub struct ProtocolLockCommand;

impl ProtocolLockCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("protocol-lock")
            .about("Protocol lock on|off|status command.")
            .settings(&[
                AppSettings::DisableHelpSubcommand,
                AppSettings::SubcommandRequired,
            ])
            .subcommand(OffSubcommand::new())
            .subcommand(OnSubcommand::new())
            .subcommand(StatusSubcommand::new())
    }
}

/// Parses the command for taking a reading from the sensor.
pub struct ReadCommand;

impl ReadCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("read")
            .about("Read command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Parses the command for putting the sensor to sleep (low-power mode).
pub struct SleepCommand;

impl SleepCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("sleep")
            .about("Sleep command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}
