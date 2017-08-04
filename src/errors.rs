use std::io;
use neuras;
use zmq;

error_chain! {
    errors {
        AddressParse {
            description ("could not parse address")
        }
        ConfigParse {
            description ("could not parse configuration file")
        }
        Neurotic {
            description ("our network has gone neurotic")
        }
        ResponseParse {
            description ("could not parse response")
        }
    }
    links {
        Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
    }
    foreign_links {
        Io(io::Error);
        Zmq(zmq::Error);
    }
}
