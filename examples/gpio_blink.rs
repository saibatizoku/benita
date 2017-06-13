extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::time::Duration;
use std::thread::sleep;

/// This is the main program. It exports and  enables blinking on an LED
/// placed on GPIO 17, aka. "Pin 11" on a Raspberry Pi with ARMv6 running
/// Raspbian. The LED blinks every 5 seconds.
fn main() {
    let my_led = Pin::new(17); // number depends on chip (e.g. Raspberry Pi
                               // model), etc.
    my_led.with_exported(|| {
        println!("Blinking pin: {:?}", my_led);
        my_led.set_direction(Direction::Low).expect("Could not specify GPIO direction");
        loop {
            println!("❤");
            my_led.set_value(1).expect("Unable to turn on LED");
            sleep(Duration::from_millis(320));
            my_led.set_value(0).expect("Unable to turn off LED");
            sleep(Duration::from_millis(4680));
        }
    }).expect("Could not export GPIO Pin");

    // Be a good command-line citizen.
    ::std::process::exit(0);
}
