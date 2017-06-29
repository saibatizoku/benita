benita
======

Utilería de software, escrita en Rust, para la gestión de cultivos acuáticos de pequeña escala.

La compilación para Raspberry Pi 3 (arquitectura ARM 64-bits), se realiza con instalando [con rustup](https://www.rust-lang.org/es-ES/install.html) normalmente desde el dispositivo. También es posible usar [rust-on-raspberry-docker](https://github.com/Ragnaroek/rust-on-raspberry-docker), siguiendo los pasos ahí establecidos desde otro equipo para compilación cruzada.

Para el funcionamiento correcto del software, es necesario que I2C esté habilitado en el dispositivo que lo ejecutará.

## Sensores acuáticos

### pH con temperatura

* pH EZO (Atlas Scientific) - I2C Mode

* RTD EZO (Atlas Scientific) - I2C Mode

### Conductividad con temperatura

* EC EZO (Atlas Scientific) - I2C Mode

* RTD EZO (Atlas Scientific) - I2C Mode

## Sensores atmosféricos

### Raspberry Pi Sense Hat 

* Barómetro - I2C
* Temperatura - I2C
* Humedad relativa - I2C
