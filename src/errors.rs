//! Library Error, and ErrorKind definitions.
use super::utilities;

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
        SensorTrouble {
            description ("trouble with the sensor")
        }
    }
    links {
        // module error-chains
        Utilities(utilities::errors::Error, utilities::errors::ErrorKind);

        // external crate error-chains
        Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
    }
}
