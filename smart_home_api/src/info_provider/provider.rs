use crate::my_smart_home::error::SmartHomeResult;
use crate::my_smart_home::home::Home;
use crate::my_smart_home::smart_home::SmartHome;
use std::collections::HashSet;

pub trait DeviceInfoProvider {
    /// метод, возвращающий отчет устройства по имени комнаты и имени устройства
    fn get_device_info(&self, home: &Home, room: &str, device: &str) -> SmartHomeResult<String> {
        match home.get_device(room, device) {
            Ok(device) => Ok(device.report()),
            Err(error) => Err(error),
        }
    }
}

pub trait IterableProvider {
    fn as_set(&self) -> HashSet<String>;
}
