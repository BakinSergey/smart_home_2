use super::device::{DeviceState, DeviceType, SmartDevice};

#[derive(Clone)]
pub struct Thermometer {
    dev_type: DeviceType,
    name: String,
    state: DeviceState,
    temperature: f32,
}

impl Thermometer {
    pub fn new(id: &str) -> Self {
        Self {
            dev_type: DeviceType::Thermometer,
            name: ("Thermometer ".to_owned() + id).to_string(),
            state: DeviceState::On,
            temperature: f32::default(),
        }
    }

    fn get_current_temperature(&self) -> f32 {
        self.temperature
    }
}

impl SmartDevice for Thermometer {
    fn get_type(&self) -> DeviceType {
        self.dev_type.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn device_state(&self) -> DeviceState {
        self.state
    }

    fn get_current_info(&self) -> String {
        let t = self.get_current_temperature();
        format!("Текущая температура: {t:.2} °C")
    }

    fn set_device_state(&mut self, device_state: DeviceState) -> String {
        match self.state {
            DeviceState::Off | DeviceState::On => {
                self.state = device_state;
                "ok, state was set".into()
            }
            DeviceState::Broken => "failed, can't change state, device is broken!".into(),
        }
    }
}
