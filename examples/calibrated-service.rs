//! An example that takes readings from the RTD EZO chip in a loop.
//!
#![recursion_limit = "1024"]
extern crate benita;

use std::fs::File;
use std::io::Read;

use benita::cli::benita::benita_calibrated_service_cli_parser;
use benita::config::SensorServiceConfig as Config;
use benita::errors::*;
use benita::network::services::run_calibrated_sampling_service;


const SUB_CHANNEL: &'static str = "temperature-0123456789abcdef";


fn parse_cli_arguments() -> Result<()> {
    let matches = benita_calibrated_service_cli_parser().get_matches();

    let mut input = String::new();
    let mut config = Config::default();
    config.channel = SUB_CHANNEL;

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", &c);
        let _read = File::open(&c)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();

        config = Config::from_str(&input)?;
    } else {
        if let Some(pub_url) = matches.value_of("pub-server-url") {
            config.pub_url = pub_url;
        }
        if let Some(rep_ec_url) = matches.value_of("rep-ec-url") {
            config.rep_ec_url = rep_ec_url;
        }
        if let Some(rep_ph_url) = matches.value_of("rep-ph-url") {
            config.rep_ph_url = rep_ph_url;
        }
    }

    let _run = run_calibrated_sampling_service(config)?;

    // Never reach this line...
    Ok(())
}

fn main() {
    if let Err(ref e) = parse_cli_arguments() {
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
