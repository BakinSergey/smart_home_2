use crate::error::SmartHomeResult;
use crate::home::SmartHome;
use std::collections::HashSet;

pub trait DeviceInfoProvider {
    /// метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_info(
        &self,
        home: &SmartHome,
        room: &str,
        device: &str,
    ) -> SmartHomeResult<String> {
        match home.get_device(room, device) {
            Ok(device) => Ok(device.report()),
            Err(error) => Err(error),
        }
    }
}

pub trait IterableProvider {
    fn as_set(&self) -> HashSet<String>;
}
