//! Library Error, and ErrorKind definitions.
use super::devices;
use super::network;
use super::utilities;

use super::conductivity;
use super::ph;
use super::temperature;

use neuras;

error_chain! {
    errors {
        AddressParse {
            description ("could not parse address")
        }
        CommandParse {
            description ("could not parse command")
        }
        CommandRequest {
            description ("command request failed")
        }
        CommandReply {
            description ("command reply failed")
        }
        CommandResponse {
            description ("command response failed")
        }
        ConfigParse {
            description ("could not parse configuration file")
        }
        SocketBind {
            description ("socket could not bind to the network URL")
        }
        SocketCreate {
            description ("the socket couldn't be created")
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
        NumberParse {
            description ("this is not a number")
        }
        RequestParse {
            description ("could not parse request")
        }
        ResponseParse {
            description ("could not parse response")
        }
    }
    links {
        // module error-chains
        Devices(devices::errors::Error, devices::errors::ErrorKind);
        Network(network::errors::Error, network::errors::ErrorKind);
        Utilities(utilities::errors::Error, utilities::errors::ErrorKind);

        Conductivity(conductivity::errors::Error, conductivity::errors::ErrorKind);
        Temperature(temperature::errors::Error, temperature::errors::ErrorKind);
        Ph(ph::errors::Error, ph::errors::ErrorKind);

        // external crate error-chains
        Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
    }
}
