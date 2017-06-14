/// I2C Commands for RTD EZO Chip, taken from their Datasheet.
/// This chip is used for temperature measurement. It features
/// calibration, sleep mode, scale, etc.
enum RtdEzoCommands {
    Baud(u16),
    CalibrationTemperature(f64),
    CalibrationClear,
    CalibrationState,
    DataloggerPeriod(u8),
    DataloggerDisable,
    DataloggerInterval,
    DeviceAddress(u8),
    DeviceInformation,
    Export,
    ExportInfo,
    Import(String),
    Factory,
    Find,
    LedOn,
    LedOff,
    LedState,
    MemoryClear,
    MemoryRecall,
    MemoryRecallLastLocation,
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockStatus,
    Reading,
    ScaleCelsius,
    ScaleKelvin,
    ScaleFahrenheit,
    ScaleStatus,
    Sleep,
    Status
}

fn temperature_command(cmd: RtdEzoCommands) -> String {
    use self::RtdEzoCommands::*;
    match cmd {
        Baud(baud) => { format!("Baud,{}", baud) },
        CalibrationTemperature(temp) => { format!("Cal,{:.*}", 2, temp) },
        CalibrationClear => { String::from("Cal,clear") },
        CalibrationState => { String::from("Cal,?") },
        DataloggerPeriod(n) => { format!("D,{}", n) },
        DataloggerDisable => { String::from("D,0") },
        DataloggerInterval => { String::from("D,?") },
        DeviceAddress(addr) => { format!("I2C,{}", addr) },
        DeviceInformation => { String::from("I") },
        Export => { String::from("Export") },
        ExportInfo => { String::from("Export,?") },
        Import(calib) => { format!("Import,{}", calib) },
        Factory => { String::from("Factory") },
        Find => { String::from("F") },
        LedOn => { String::from("L,1") },
        LedOff => { String::from("L,0") },
        LedState => { String::from("L,?") },
        MemoryClear => { String::from("M,clear") },
        MemoryRecall => { String::from("M") },
        MemoryRecallLastLocation => { String::from("M,?") },
        ProtocolLockEnable => { String::from("Plock,1") },
        ProtocolLockDisable => { String::from("Plock,0") },
        ProtocolLockStatus => { String::from("Plock,?") },
        Reading => { String::from("R") },
        ScaleCelsius => { String::from("S,c") },
        ScaleKelvin => { String::from("S,k") },
        ScaleFahrenheit => { String::from("S,f") },
        ScaleStatus => { String::from("S,?") },
        Sleep => { String::from("Sleep") },
        Status => { String::from("Status") },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::RtdEzoCommands::*;

    #[test]
    fn temperature_command_uart_mode() {
        let cmd= temperature_command(Baud(9600));
        assert_eq!(cmd, "Baud,9600");
    }

    #[test]
    fn temperature_command_calibration_temperature() {
        let cmd= temperature_command(CalibrationTemperature(35.2459));
        assert_eq!(cmd, "Cal,35.25");
    }

    #[test]
    fn temperature_command_calibration_clear() {
        let cmd= temperature_command(CalibrationClear);
        assert_eq!(cmd, "Cal,clear");
    }

    #[test]
    fn temperature_command_calibration_state() {
        let cmd= temperature_command(CalibrationState);
        assert_eq!(cmd, "Cal,?");
    }

    #[test]
    fn temperature_command_data_logger_period() {
        let cmd= temperature_command(DataloggerPeriod(10));
        assert_eq!(cmd, "D,10");
    }

    #[test]
    fn temperature_command_data_logger_disable() {
        let cmd= temperature_command(DataloggerDisable);
        assert_eq!(cmd, "D,0");
    }

    #[test]
    fn temperature_command_data_logger_interval() {
        let cmd= temperature_command(DataloggerInterval);
        assert_eq!(cmd, "D,?");
    }

    #[test]
    fn temperature_command_() {
        let cmd= temperature_command(DeviceAddress(88));
        assert_eq!(cmd, "I2C,88");
    }

    #[test]
    fn temperature_command_device_information() {
        let cmd= temperature_command(DeviceInformation);
        assert_eq!(cmd, "I");
    }

    #[test]
    fn temperature_command_export() {
        let cmd= temperature_command(Export);
        assert_eq!(cmd, "Export");
    }

    #[test]
    fn temperature_command_export_info() {
        let cmd= temperature_command(ExportInfo);
        assert_eq!(cmd, "Export,?");
    }

    #[test]
    fn temperature_command_import() {
        let cmd= temperature_command(Import(String::from("abcdef")));
        assert_eq!(cmd, "Import,abcdef");
    }

    #[test]
    fn temperature_command_factory() {
        let cmd= temperature_command(Factory);
        assert_eq!(cmd, "Factory");
    }

    #[test]
    fn temperature_command_find() {
        let cmd= temperature_command(Find);
        assert_eq!(cmd, "F");
    }

    #[test]
    fn temperature_command_led_on() {
        let cmd= temperature_command(LedOn);
        assert_eq!(cmd, "L,1");
    }

    #[test]
    fn temperature_command_led_off() {
        let cmd= temperature_command(LedOff);
        assert_eq!(cmd, "L,0");
    }

    #[test]
    fn temperature_command_led_state() {
        let cmd= temperature_command(LedState);
        assert_eq!(cmd, "L,?");
    }

    #[test]
    fn temperature_command_memory_clear() {
        let cmd= temperature_command(MemoryClear);
        assert_eq!(cmd, "M,clear");
    }

    #[test]
    fn temperature_command_memory_recall() {
        let cmd= temperature_command(MemoryRecall);
        assert_eq!(cmd, "M");
    }

    #[test]
    fn temperature_command_memory_recall_location() {
        let cmd= temperature_command(MemoryRecallLastLocation);
        assert_eq!(cmd, "M,?");
    }

    #[test]
    fn temperature_command_plock_enable() {
        let cmd= temperature_command(ProtocolLockEnable);
        assert_eq!(cmd, "Plock,1");
    }

    #[test]
    fn temperature_command_plock_disable() {
        let cmd= temperature_command(ProtocolLockDisable);
        assert_eq!(cmd, "Plock,0");
    }

    #[test]
    fn temperature_command_plock_status() {
        let cmd= temperature_command(ProtocolLockStatus);
        assert_eq!(cmd, "Plock,?");
    }

    #[test]
    fn temperature_command_reading() {
        let cmd= temperature_command(Reading);
        assert_eq!(cmd, "R");
    }

    #[test]
    fn temperature_command_scale_celsius() {
        let cmd= temperature_command(ScaleCelsius);
        assert_eq!(cmd, "S,c");
    }

    #[test]
    fn temperature_command_scale_kelvin() {
        let cmd= temperature_command(ScaleKelvin);
        assert_eq!(cmd, "S,k");
    }

    #[test]
    fn temperature_command_scale_fahrenheit() {
        let cmd= temperature_command(ScaleFahrenheit);
        assert_eq!(cmd, "S,f");
    }

    #[test]
    fn temperature_command_scale_status() {
        let cmd= temperature_command(ScaleStatus);
        assert_eq!(cmd, "S,?");
    }

    #[test]
    fn temperature_command_sleep_mode() {
        let cmd= temperature_command(Sleep);
        assert_eq!(cmd, "Sleep");
    }

    #[test]
    fn temperature_command_device_status() {
        let cmd= temperature_command(Status);
        assert_eq!(cmd, "Status");
    }
}
