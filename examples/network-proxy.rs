extern crate benita;
extern crate clap;
extern crate zmq;

use std::fs::File;
use std::io::Read;

use benita::errors::*;
use benita::ProxyConfig;
use benita::neuras::zmq_xpub_xsub_proxy;
use clap::{App, Arg};

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-neuras-proxy")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. XPUB-XSUB proxy")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .arg(Arg::with_name("backend-url")
                 .short("b")
                 .long("backend")
                 .value_name("BACKEND_URL")
                 .help("Sets the url for the backend server")
                 .takes_value(true)
                 .index(1)
                 .required(true)
                 .conflicts_with_all(&["config"]))
        .arg(Arg::with_name("frontend-url")
                 .short("f")
                 .long("frontend")
                 .value_name("FRONTEND_URL")
                 .help("Sets the url for the frontend server")
                 .takes_value(true)
                 .required(true)
                 .index(2)
                 .conflicts_with_all(&["config"]))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .multiple(true)
                 .help("Turn debugging information on"))
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
            config.frontend = frontend;
        }

        if let Some(backend) = matches.value_of("backend-url") {
            config.backend = backend;
        }
    }

    let _run =  run_proxy(config.backend, config.frontend)?;

    // Never reach this line...
    Ok(())
}

fn run_proxy(backend: &str, frontend: &str) -> Result<()> {
    let context = zmq::Context::new();
    println!("Proxied PUB service now serving at: {}", &frontend);
    println!("... press `Ctrl-C` to quit.");
    zmq_xpub_xsub_proxy(&context, backend, frontend)
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
