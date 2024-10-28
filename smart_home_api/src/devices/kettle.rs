use super::device::{DeviceState, DeviceType, SmartDevice};

#[derive(Clone)]
pub struct Kettle {
    dev_type: DeviceType,
    name: String,
    temperature: f32,
    water_volume: f32,
    state: DeviceState,
}

impl Kettle {
    pub fn new(id: &str) -> Self {
        Self {
            dev_type: DeviceType::Kettle,
            name: ("Smart Kettle ".to_owned() + id).to_string(),
            temperature: 0.0,
            water_volume: 1.1,
            state: DeviceState::Off,
        }
    }

    pub fn switch_on(&mut self) {
        self.state = DeviceState::On;
    }

    pub fn switch_off(&mut self) {
        self.state = DeviceState::Off;
    }

    fn get_current_temperature(&self) -> f32 {
        self.temperature
    }

    fn get_current_water_volume(&self) -> f32 {
        self.water_volume
    }
}

impl SmartDevice for Kettle {
    fn get_type(&self) -> DeviceType {
        self.dev_type.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn device_state(&self) -> DeviceState {
        self.state
    }

    fn get_current_state(&self) -> String {
        let t = self.get_current_temperature();
        let wv = self.get_current_water_volume();
        format!("В чайнике: {wv:.2} л воды, Текущая температура: {t:.2} °C")
    }
}
