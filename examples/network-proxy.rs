//! Proxy between networks for PUB services.
//!
extern crate benita;
extern crate chrono;
extern crate clap;
#[macro_use]
extern crate failure;
extern crate fern;
#[macro_use]
extern crate log;
extern crate neuras;

use std::fs::File;
use std::io::Read;

use benita::ezo::errors::*;
use benita::ezo::config::ProxyConfig;
use clap::{App, Arg};
use neuras::utils::{create_context, zmq_xpub_xsub_proxy};

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
        .chain(fern::log_file("proxy.log")
            .chain_err(|| "failed to open log file")?)
        .apply()
        .chain_err(|| "Could not setup logging")?;
    Ok(())
}

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-neuras-proxy")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. XPUB-XSUB proxy")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("backend-url")
                .short("b")
                .long("backend")
                .value_name("BACKEND_URL")
                .help("Sets the url for the backend server")
                .takes_value(true)
                .index(1)
                .required(true)
                .conflicts_with_all(&["config"]),
        )
        .arg(
            Arg::with_name("frontend-url")
                .short("f")
                .long("frontend")
                .value_name("FRONTEND_URL")
                .help("Sets the url for the frontend server")
                .takes_value(true)
                .required(true)
                .index(2)
                .conflicts_with_all(&["config"]),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .multiple(true)
                .help("Turn debugging information on"),
        )
        .get_matches();

    let mut input = String::new();
    let mut config = ProxyConfig::default();

    if let Some(c) = matches.value_of("config") {
        let _read = File::open(&c)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();

        config = ProxyConfig::from_str(&input)?;
    } else {
        if let Some(frontend) = matches.value_of("frontend-url") {
            config.frontend_url = frontend;
        }

        if let Some(backend) = matches.value_of("backend-url") {
            config.backend_url = backend;
        }
    }

    let _run = run_proxy(config.backend_url, config.frontend_url)?;

    // Never reach this line...
    Ok(())
}

fn run_proxy(backend: &str, frontend: &str) -> Result<()> {
    let context = create_context();
    info!("Proxied PUB service now serving at: {}", &frontend);
    let _proxy = zmq_xpub_xsub_proxy(&context, backend, frontend)?;
    Ok(())
}

fn run_code() -> Result<()> {
    // Initialize logging.
    let _log = start_logger()?;
    info!("Starting network-proxy");
    parse_cli_arguments()
}

quick_main!(run_code);
