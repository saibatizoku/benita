//! An example that takes readings from the RTD EZO chip in a loop.
//!
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate fern;
#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Read;

use benita::cli::benita::benita_calibrated_service_cli_parser;
use benita::config::SensorServiceConfig;
use benita::errors::*;
use benita::services::calibrated::run_calibrated_sampling_service;


const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";


fn evaluate_command_line() -> Result<()> {
    // Use the pre-configured `clap` application for this service to read from stdin.
    let matches = benita_calibrated_service_cli_parser().get_matches();

    // The sensor service needs to be configured either from a file or from
    // command-line arguments
    let mut path_arg = "".to_string();
    let config = if let Some(c) = matches.value_of("config") {
        debug!("parsing config from file: {}", &c);
        let _read = File::open(&c)
            .and_then(|mut f| f.read_to_string(&mut path_arg))
            .chain_err(|| "Export ")?;

        SensorServiceConfig::from_str(&path_arg)?
    } else {
        debug!("parsing config from cli arguments");
        SensorServiceConfig {
            channel: SUB_CHANNEL,
            pub_url: matches.value_of("pub-server-url").unwrap(),
            rep_ec_url: matches.value_of("rep-ec-url").unwrap(),
            rep_ph_url: matches.value_of("rep-ph-url").unwrap(),
        }
    };
    debug!("configured publication URL");
    debug!("configured conductivity requester URL");
    debug!("configured pH requester URL");

    info!("Running Calibrated Service");
    let _run = run_calibrated_sampling_service(config)?;

    // Never reach this line...
    Ok(())
}

// Configure and start logger.
fn start_logger() -> Result<()> {
    let _logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("calibrated-service.log")
            .chain_err(|| "failed to open log file")?)
        .apply()
        .chain_err(|| "Could not setup logging")?;
    Ok(())
}

// Main program. Starts logger, then evaluates args from stdin.
fn run_code() -> Result<()> {
    // Initialize logging.
    let _log = start_logger()?;
    info!("Starting calibrated-service");
    evaluate_command_line()
}

// fn main() wrapped to handle error chains
quick_main!(run_code);
