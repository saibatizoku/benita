//! Sends remote commands to the RTD EZO chip, using the exposed a limited API.
//!
//! This client binds to `tcp://localhost:5557`.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate zmq;

use benita::errors::*;
use benita::neuras::{zmq_req, connect_client};

fn run() -> Result<()> {
    let context = zmq::Context::new();
    let requester = zmq_req(&context)?;

    let _connect = connect_client(&requester, "tcp://192.168.16.123:5557")?;

    let mut msg = zmq::Message::new().unwrap();

    {
        println!("Requesting 'get_output_params'");
        requester.send("get_params".as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());

        println!("Requesting 'read'");
        requester.send("read".as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());

        println!("Requesting 'sleep'");
        requester.send("sleep".as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());
    }
    Ok(())
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
