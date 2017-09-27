//! Networked services for Conductivity sensing.
use errors::*;

use neuras;

/// REP command-set.
///
/// *   `calibrate n` command, where n is a temperature float/int.
/// *   `get_params` command, return the output readings configuration.
/// *   `read` command, returns the output readings.
/// *   `sleep` command, sets the device to sleep/low-power mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum REPCommand {
    Calibrate(f64),
    GetParams,
    Read,
    Sleep,
}

impl REPCommand {
    pub fn parse(cmd_str: &str) -> Result<REPCommand> {
        let cmd = match cmd_str {
            "read" => REPCommand::Read,
            a if cmd_str.starts_with("calibrate ") => {
                let rest = a.get(10..).unwrap();
                let temp = rest.parse().unwrap();
                REPCommand::Calibrate(temp)
            }
            "get_params" => REPCommand::GetParams,
            "sleep" => REPCommand::Sleep,
            _ => return Err(ErrorKind::CommandParse.into()),
        };
        Ok(cmd)
    }
}


        }
        }
        }
    }

}


    }
}
