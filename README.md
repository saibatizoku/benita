benita [![Build Status](https://travis-ci.org/saibatizoku/benita.svg?branch=master)](https://travis-ci.org/saibatizoku/benita)
======

# Talk directly to your sensor, via `i2cdev`.

Run `benita` on your `Raspberry Pi` (currently tested on RPi 3), hookup your sensors to the I2C bus, and start coding!

## Quickstart

Initialize pre-configured sensors, just say which `i2cdev` path, and which `I2C Address` to call on.

At the moment, _conductivity_, _temperature_, and _ph_ sensors are available.

The simplest way to start talking to a sensor is to use the `new(path, address)` associated function on the device.

### A Simple pH sensor

```norun
extern crate benita;

use benita::config::SensorConfig;
use benita::errors::*;
use benita::devices::ph::PhSensor;

fn main() {
    // Create a sensor directly with the i2cdev path, and the integer value
    // of the I2C address for our sensor. It must be mutable
    // to write to the I2C bus.
    let mut ph_sensor = PhSensor::new("/dev/i2cdev-0", 100)?;

    // use the sensor to issue a command request and display the response.
    let response = ph_sensor.get_device_info()?;
    println!("Sensor Info: {}", response);

    // custom code goes here...
}
```

### pH Sensor with typed configuration
```norun
extern crate benita;

use benita::config::SensorConfig;
use benita::errors::*;
use benita::devices::ph::PhSensor;


fn main() {

    // Same goes for...
    // First, create a sensor configuration with the i2cdev path, and the
    // integer value of the I2C address for our sensor.
    let ph_config = PhSensor::new("/dev/i2cdev-0", 99)?;

    // Second, we can start working with our sensor device. It must be mutable
    // to write to the I2C bus.
    let mut ph_sensor = PhSensor::from_config(ph_config)?;

    // use the sensor to issue a command request and display the response.
    let response = ph_sensor.get_reading()?;
    println!("Current pH: {}", response);

    // custom code goes here...
}
```
