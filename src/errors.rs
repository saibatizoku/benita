//! Create the Error, ErrorKind, ResultExt, and Result types.
use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
    }
}
