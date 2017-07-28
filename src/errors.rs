//! Create the Error, ErrorKind, ResultExt, and Result types.
use std::io;

error_chain! {
    errors {
        // The address could not be parsed
        AddressParse {
            description ("could not parse address")
        }
        // Trouble with neuras
        Neurotic {
            description ("our network has gone neurotic")
        }
        // The response could not be parsed
        ResponseParse {
            description ("could not parse response")
        }
    }
    foreign_links {
        Io(io::Error);
    }
}
