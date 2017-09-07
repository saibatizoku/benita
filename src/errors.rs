use std::io;
use neuras;
use ezo_ec;
use ezo_ph;
use ezo_rtd;

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
        SensorTrouble {
            description ("trouble with the sensor")
        }
    }
    links {
        Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
        EcSensor(ezo_ec::errors::Error, ezo_ec::errors::ErrorKind);
        PhSensor(ezo_ph::errors::Error, ezo_ph::errors::ErrorKind);
        RtdSensor(ezo_rtd::errors::Error, ezo_rtd::errors::ErrorKind);
    }
    foreign_links {
        Io(io::Error);
    }
}
