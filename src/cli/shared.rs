//! Shared subcommands and utility functions.
use std;

use errors::*;
use utilities::atof;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use neuras::url::Url;

/// Validator function for URLs.
pub fn is_url(v: String) -> std::result::Result<(), String> {
    match Url::parse(&v) {
        Ok(_) => Ok(()),
        _ => Err("Invalid URL".to_string()),
    }
}

/// Validator function for floating point numbers.
pub fn is_float(v: String) -> std::result::Result<(), String> {
    match v.parse::<f64>() {
        Ok(_) => Ok(()),
        _ => Err("The value is not numeric.".to_string()),
    }
}

/// `compensation set <TEMP>` command
pub struct SetSubcommand(pub f64);

impl SetSubcommand {
    /// Command-line matcher for setting temperature compensation of readings.
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("set")
            .about("Set compensation temperature value.")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("TEMP")
                    .help("Numeric value up to 3 decimals.")
                    .takes_value(true)
                    .validator(is_float)
                    .required(true),
            )
    }

    /// Parses the `ArgMatches` and returns a `SetSubcommand` instance.
    pub fn parse_args(cli: &mut App, args: &[&str]) -> Result<SetSubcommand> {
        let matches = cli.get_matches_from_safe_borrow(args)
            .chain_err(|| "no match")?;
        let temp = match matches.value_of("TEMP") {
            Some(t) => atof(t)?,
            _ => unreachable!(),
        };
        Ok(SetSubcommand(temp))
    }

    /// Returns the command as a `String`.
    pub fn to_string(&self) -> String {
        format!("set {:.*}", 3, self.0)
    }
}

/// `compensation get` command
pub struct GetSubcommand;

impl GetSubcommand {
    /// Command-line matcher for setting temperature compensation of readings.
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("get")
            .about("Set compensation temperature value.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }

    /// Parses the `ArgMatches` and returns a `GetSubcommand` instance.
    pub fn parse_args(_matches: &ArgMatches) -> Result<GetSubcommand> {
        Ok(GetSubcommand)
    }

    /// Returns the command as a `&str`.
    pub fn to_string(&self) -> String {
        "get".to_string()
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

pub struct ClearSubcommand;

impl ClearSubcommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("clear")
            .about("Clear calibration settings command.")
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
                    .required(true),
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
