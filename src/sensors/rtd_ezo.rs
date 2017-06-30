/// I2C Commands for RTD EZO Chip, taken from their Datasheet.
/// This chip is used for temperature measurement. It features
/// calibration, sleep mode, scale, etc.

use I2cCommand;

pub enum RtdEzoCommand {
    CalibrationTemperature(f64),
    CalibrationClear,
    CalibrationState,
    DataloggerPeriod(u16),
    DataloggerDisable,
    DataloggerInterval,
    DeviceAddress(u16),
    DeviceInformation,
    Export(String),
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
    SetUart(Bauds),
    Sleep,
    Status,
}

pub enum Bauds {
    Bps300,
    Bps1200,
    Bps2400,
    Bps9600,
    Bps19200,
    Bps38400,
    Bps57600,
    Bps115200,
}

impl I2cCommand for RtdEzoCommand {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    fn to_string(&self) -> String {
        use self::RtdEzoCommand::*;
        match *self {
            CalibrationTemperature(temp) => format!("Cal,{:.*}\0", 2, temp),
            CalibrationClear => "Cal,clear\0".to_string(),
            CalibrationState => "Cal,?\0".to_string(),
            DataloggerPeriod(n) => format!("D,{}\0", n),
            DataloggerDisable => "D,0\0".to_string(),
            DataloggerInterval => "D,?\0".to_string(),
            DeviceAddress(addr) => format!("I2C,{}\0", addr),
            DeviceInformation => "I\0".to_string(),
            Export(ref calib) => format!("Export,{}\0", calib),
            ExportInfo => "Export,?\0".to_string(),
            Import(ref calib) => format!("Import,{}\0", calib),
            Factory => "Factory\0".to_string(),
            Find => "F\0".to_string(),
            LedOn => "L,1\0".to_string(),
            LedOff => "L,0\0".to_string(),
            LedState => "L,?\0".to_string(),
            MemoryClear => "M,clear\0".to_string(),
            MemoryRecall => "M\0".to_string(),
            MemoryRecallLastLocation => "M,?\0".to_string(),
            ProtocolLockEnable => "Plock,1\0".to_string(),
            ProtocolLockDisable => "Plock,0\0".to_string(),
            ProtocolLockStatus => "Plock,?\0".to_string(),
            Reading => "R\0".to_string(),
            ScaleCelsius => "S,c\0".to_string(),
            ScaleKelvin => "S,k\0".to_string(),
            ScaleFahrenheit => "S,f\0".to_string(),
            ScaleStatus => "S,?\0".to_string(),
            SetUart(ref baud) => {
                let rate = match *baud {
                    Bauds::Bps300 => 300,
                    Bauds::Bps1200 => 1200,
                    Bauds::Bps2400 => 2400,
                    Bauds::Bps9600 => 9600,
                    Bauds::Bps19200 => 19200,
                    Bauds::Bps38400 => 38400,
                    Bauds::Bps57600 => 57600,
                    Bauds::Bps115200 => 115200,
                };
                format!("Baud,{}\0", rate)
            },
            Sleep => "Sleep\0".to_string(),
            Status => "Status\0".to_string(),
        }
    }
}

fn temperature_command(cmd: RtdEzoCommand) -> String {
    cmd.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::RtdEzoCommand::*;

    #[test]
    fn temperature_command_uart_mode() {
        let cmd = temperature_command(SetUart(Bauds::Bps9600));
        assert_eq!(cmd, "Baud,9600\0");
    }

    #[test]
    fn temperature_command_calibration_temperature() {
        let cmd = temperature_command(CalibrationTemperature(35.2459));
        assert_eq!(cmd, "Cal,35.25\0");
    }

    #[test]
    fn temperature_command_calibration_clear() {
        let cmd = temperature_command(CalibrationClear);
        assert_eq!(cmd, "Cal,clear\0");
    }

    #[test]
    fn temperature_command_calibration_state() {
        let cmd = temperature_command(CalibrationState);
        assert_eq!(cmd, "Cal,?\0");
    }

    #[test]
    fn temperature_command_data_logger_period() {
        let cmd = temperature_command(DataloggerPeriod(10));
        assert_eq!(cmd, "D,10\0");
    }

    #[test]
    fn temperature_command_data_logger_disable() {
        let cmd = temperature_command(DataloggerDisable);
        assert_eq!(cmd, "D,0\0");
    }

    #[test]
    fn temperature_command_data_logger_interval() {
        let cmd = temperature_command(DataloggerInterval);
        assert_eq!(cmd, "D,?\0");
    }

    #[test]
    fn temperature_command_() {
        let cmd = temperature_command(DeviceAddress(88));
        assert_eq!(cmd, "I2C,88\0");
    }

    #[test]
    fn temperature_command_device_information() {
        let cmd = temperature_command(DeviceInformation);
        assert_eq!(cmd, "I\0");
    }

    #[test]
    fn temperature_command_export() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = temperature_command(Export(calibration_string));
        assert_eq!(cmd, "Export,ABCDEFGHIJKLMNO\0");
    }

    #[test]
    fn temperature_command_export_info() {
        let cmd = temperature_command(ExportInfo);
        assert_eq!(cmd, "Export,?\0");
    }

    #[test]
    fn temperature_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = temperature_command(Import(calibration_string));
        assert_eq!(cmd, "Import,ABCDEFGHIJKLMNO\0");
    }

    #[test]
    fn temperature_command_factory() {
        let cmd = temperature_command(Factory);
        assert_eq!(cmd, "Factory\0");
    }

    #[test]
    fn temperature_command_find() {
        let cmd = temperature_command(Find);
        assert_eq!(cmd, "F\0");
    }

    #[test]
    fn temperature_command_led_on() {
        let cmd = temperature_command(LedOn);
        assert_eq!(cmd, "L,1\0");
    }

    #[test]
    fn temperature_command_led_off() {
        let cmd = temperature_command(LedOff);
        assert_eq!(cmd, "L,0\0");
    }

    #[test]
    fn temperature_command_led_state() {
        let cmd = temperature_command(LedState);
        assert_eq!(cmd, "L,?\0");
    }

    #[test]
    fn temperature_command_memory_clear() {
        let cmd = temperature_command(MemoryClear);
        assert_eq!(cmd, "M,clear\0");
    }

    #[test]
    fn temperature_command_memory_recall() {
        let cmd = temperature_command(MemoryRecall);
        assert_eq!(cmd, "M\0");
    }

    #[test]
    fn temperature_command_memory_recall_location() {
        let cmd = temperature_command(MemoryRecallLastLocation);
        assert_eq!(cmd, "M,?\0");
    }

    #[test]
    fn temperature_command_plock_enable() {
        let cmd = temperature_command(ProtocolLockEnable);
        assert_eq!(cmd, "Plock,1\0");
    }

    #[test]
    fn temperature_command_plock_disable() {
        let cmd = temperature_command(ProtocolLockDisable);
        assert_eq!(cmd, "Plock,0\0");
    }

    #[test]
    fn temperature_command_plock_status() {
        let cmd = temperature_command(ProtocolLockStatus);
        assert_eq!(cmd, "Plock,?\0");
    }

    #[test]
    fn temperature_command_reading() {
        let cmd = temperature_command(Reading);
        assert_eq!(cmd, "R\0");
    }

    #[test]
    fn temperature_command_scale_celsius() {
        let cmd = temperature_command(ScaleCelsius);
        assert_eq!(cmd, "S,c\0");
    }

    #[test]
    fn temperature_command_scale_kelvin() {
        let cmd = temperature_command(ScaleKelvin);
        assert_eq!(cmd, "S,k\0");
    }

    #[test]
    fn temperature_command_scale_fahrenheit() {
        let cmd = temperature_command(ScaleFahrenheit);
        assert_eq!(cmd, "S,f\0");
    }

    #[test]
    fn temperature_command_scale_status() {
        let cmd = temperature_command(ScaleStatus);
        assert_eq!(cmd, "S,?\0");
    }

    #[test]
    fn temperature_command_sleep_mode() {
        let cmd = temperature_command(Sleep);
        assert_eq!(cmd, "Sleep\0");
    }

    #[test]
    fn temperature_command_device_status() {
        let cmd = temperature_command(Status);
        assert_eq!(cmd, "Status\0");
    }
}
