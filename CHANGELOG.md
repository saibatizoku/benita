## [0.1.6] - 2017-11-05
### Added
- refactored examples for clarity
- responder examples don't crash with sensor errors
- replace `OkReply` with `ReplyStatus`, an enum that maybe `Ok` or `Err`.
- enable logging for crate and examples
- refactored calibrated service a bit
## [0.1.5] - 2017-11-02
### Added
- specify Sensor APIs as traits
- requesters implement sensor APIs
- tests for all sensor replies
- services continue to work using APIs... kinda
## [0.1.4] - 2017-10-31
### Added
- shaping up REP-REQ sensor APIs for: conductivity, ph, and temperature.
### Added
## [0.1.3] - 2017-10-30
### Added
- Mechanism and traits for handling requests and replies
- Major overhaul of item names... sorry.
- Renamed most macros to better reflect what they do
## [0.1.2] - 2017-10-23
### Added
- Network requests for pH sensor.
- Network replies for pH sensor.
- Network requests for Temperature sensor.
- Network replies for Temperature sensor.
## [0.1.1] - 2017-10-23
### Added
- Network requests for Conductivity sensor.
- Network replies for Conductivity sensor.
