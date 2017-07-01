benita
======

Utilería de software, escrita en Rust, para la gestión de cultivos acuáticos de pequeña escala.

La compilación para Raspberry Pi 3 (arquitectura ARM 64-bits), se realiza con instalando [con rustup](https://www.rust-lang.org/es-ES/install.html) normalmente desde el dispositivo. También es posible usar [rust-on-raspberry-docker](https://github.com/Ragnaroek/rust-on-raspberry-docker), siguiendo los pasos ahí establecidos desde otro equipo para compilación cruzada.

Para el funcionamiento correcto del software, es necesario que I2C esté habilitado en el dispositivo que lo ejecutará.

## Dependencias

*   [ezo-ec-rs](https://github.com/saibatizoku/ezo-ec-rs) - Sensor sumergible de conductividad eléctrica.
*   [ezo-ph-rs](https://github.com/saibatizoku/ezo-ph-rs) - Sensor sumergible de pH.
*   [ezo-rtd-rs](https://github.com/saibatizoku/ezo-rtd-rs) - Sensor sumergible de temperatura.


*   [neuras](https://github.com/saibatizoku/neuras) - "Chapopote para redes sensoriales"


*   [error-chain](https://github.com/brson/error-chain)
*   [rust-i2cdev](https://github.com/rust-embedded/rust-i2cdev)
*   [rust-sysfs-gpio](https://github.com/rust-embedded/rust-sysfs-gpio)
*   [sensehat-rs](https://github.com/shaunmulligan/sensehat-rs.git)

## Arquitectura del hardware

*   `Single-board computer (SBC)` - Raspberry Pi 3
*   Sensores sumergibles - Chips EZO (Atlas Scientific)
    *   `I2C address: 0x63` - pH
    *   `I2C address: 0x64` - Conductividad eléctrica
    *   `I2C address: 0x65` - Temperatura
*   Sensores atmosféricos (Raspberry Pi Sense Hat )
    * `I2C address: 0x5c` - Barómetro
        1. Barómetro
        2. Temperatura
    * `I2C address: 0x5f` - Humedad relativa
        1. Humedad relativa
        2. Temperatura
