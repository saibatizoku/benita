extern crate benita;
extern crate zmq;

use benita::errors::*;
use benita::neuras::zmq_xpub_xsub_proxy;

fn run() -> Result<()> {
    let context = zmq::Context::new();
    zmq_xpub_xsub_proxy(&context, "tcp://192.168.16.123:5556", "tcp://*:5558")
}

fn main() {
    if let Err(ref e) = run() {
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
