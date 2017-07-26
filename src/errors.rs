//! Create the Error, ErrorKind, ResultExt, and Result types.
use std::io;

error_chain! {
    errors {
        // The response could not be parsed
        ResponseParse {
            description ("could not parse response")
        }
    }
    foreign_links {
        Io(io::Error);
    }
}
