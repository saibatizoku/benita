//! Create the Error, ErrorKind, ResultExt, and Result types.
use std::io;
use zmq;
error_chain! {
    errors {
        AddressParse {
            description ("could not parse address")
        }
        Neurotic {
            description ("our network has gone neurotic")
        }
        ResponseParse {
            description ("could not parse response")
        }
    }
    foreign_links {
        Io(io::Error);
        Zmq(zmq::Error);
    }
}
