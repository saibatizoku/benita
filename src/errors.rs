//! Errors.
use std::io;
use neuras;
use ezo_common;
use ezo_ec;
use ezo_ph;
use ezo_rtd;

error_chain! {
    errors {
        AddressParse {
            description ("could not parse address")
        }
        CommandParse {
            description ("could not parse command")
        }
        ConfigParse {
            description ("could not parse configuration file")
        }
        SocketBind {
            description ("socket could not bind to the network URL")
        }
        SocketConnect {
            description ("socket could not connect to the network URL")
        }
        SocketSend {
            description ("message could not be sent to the network")
        }
        SocketReceive {
            description ("message could not be received from the network")
        }
        Neurotic {
            description ("our network has gone neurotic")
        }
        ResponseParse {
            description ("could not parse response")
        }
        SensorTrouble {
            description ("trouble with the sensor")
        }
    }
    links {
        Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
        EzoSensor(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
        EcSensor(ezo_ec::errors::Error, ezo_ec::errors::ErrorKind);
        PhSensor(ezo_ph::errors::Error, ezo_ph::errors::ErrorKind);
        RtdSensor(ezo_rtd::errors::Error, ezo_rtd::errors::ErrorKind);
    }
    foreign_links {
        Io(io::Error);
    }
}
