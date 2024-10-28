use std::fmt;

pub const NO_INFO_PROVIDED: &str = "No information provided";

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DeviceType {
    #[default]
    Socket,
    Kettle,
    Thermometer,
}

/// Статус устройства
#[derive(Copy, Clone, Default)]
pub enum DeviceState {
    On,
    #[default]
    Off,
    Broken,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceState::On => write!(f, "\x1b[32mВключен\x1b[0m"),
            DeviceState::Off => write!(f, "\x1b[33mВыключен\x1b[0m"),
            DeviceState::Broken => write!(f, "\x1b[41mНе исправен\x1b[0m"),
        }
    }
}

pub trait SmartDevice {
    /// тип устройства
    fn get_type(&self) -> DeviceType;

    /// имя устройства
    fn get_name(&self) -> String {
        format!("{} {}", NO_INFO_PROVIDED, "about name").to_string()
    }

    /// описание устройства
    fn get_description(&self) -> String {
        NO_INFO_PROVIDED.to_string()
    }

    /// описание текущего состояния
    fn device_state(&self) -> DeviceState {
        DeviceState::default()
    }

    /// текущие параметры устройства
    fn get_current_state(&self) -> String {
        NO_INFO_PROVIDED.to_string()
    }

    fn report(&self) -> String {
        format!(
            "Устройство: {}
    Описание: {}
    Состояние: {}
    Текущие параметры: {}",
            self.get_name(),
            self.get_description(),
            self.device_state(),
            self.get_current_state()
        )
    }
}

pub type VecOfDevice = Vec<Box<dyn SmartDevice>>;
