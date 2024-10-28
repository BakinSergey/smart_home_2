use super::device::{DeviceState, DeviceType, SmartDevice};

#[derive(Clone, Default)]
pub struct Socket {
    dev_type: DeviceType,
    name: String,
    description: String,
    state: DeviceState,
    power: f32,
}

impl Socket {
    pub fn new(id: &str) -> Self {
        Self {
            dev_type: DeviceType::Socket,
            name: ("Smart Socket ".to_owned() + id).to_string(),
            description: "Very Powerful Smart Device".to_string(),
            state: DeviceState::Off,
            power: 15.2,
        }
    }

    pub fn switch_on(&mut self) {
        self.state = DeviceState::On;
    }

    pub fn switch_off(&mut self) {
        self.state = DeviceState::Off;
    }

    pub fn set_broken(&mut self) {
        self.state = DeviceState::Broken;
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_current_power(&self) -> f32 {
        self.power
    }
}

impl SmartDevice for Socket {
    fn get_type(&self) -> DeviceType {
        self.dev_type.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        self.get_description()
    }

    fn device_state(&self) -> DeviceState {
        self.state
    }

    fn get_current_state(&self) -> String {
        let pw = self.get_current_power();
        format!("Текущая мощность: {pw:.2} Вт")
    }
}
