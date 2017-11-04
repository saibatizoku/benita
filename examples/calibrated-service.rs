//! An example that takes readings from the RTD EZO chip in a loop.
//!
#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate fern;

use std::fs::File;
use std::io::Read;

use benita::cli::benita::benita_calibrated_service_cli_parser;
use benita::config::SensorServiceConfig as SensorConfig;
use benita::errors::*;
use benita::services::calibrated::run_calibrated_sampling_service;


const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";


fn evaluate_command_line() -> Result<()> {
    // Use the pre-configured `clap` application for this service to read from stdin.
    let matches = benita_calibrated_service_cli_parser().get_matches();

    let mut input = String::new();
    let mut config = SensorConfig { channel: SUB_CHANNEL, ..Default::default() };

    if let Some(c) = matches.value_of("config") {
        debug!("parsing config from cli: {}", &c);
        let _read = File::open(&c)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();

        config = SensorConfig::from_str(&input)?;
    } else {
        if let Some(pub_url) = matches.value_of("pub-server-url") {
            config.pub_url = pub_url;
            debug!("configured publication URL");
        }
        if let Some(rep_ec_url) = matches.value_of("rep-ec-url") {
            config.rep_ec_url = rep_ec_url;
            debug!("configured conductivity requester URL");
        }
        if let Some(rep_ph_url) = matches.value_of("rep-ph-url") {
            config.rep_ph_url = rep_ph_url;
            debug!("configured pH requester URL");
        }
    }

    info!("Running Calibrated Service");
    let _run = run_calibrated_sampling_service(config)?;

    // Never reach this line...
    Ok(())
}

fn run_code() -> Result<()> {
    // Initialize logging.
    fern::Dispatch::new()
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
        .chain(fern::log_file("calibrated_service.log")
               .chain_err(|| "failed to open log file")?)
        .apply()
        .chain_err(|| "Could not setup logging")?;
    info!("Starting");
    evaluate_command_line()
}

fn main() {
    if let Err(ref e) = run_code() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}
