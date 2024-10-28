use crate::error::{SmartHomeError, SmartHomeResult};
use crate::home::SmartHome;
use std::collections::HashSet;

pub trait DeviceInfoProvider {
    /// метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_info(
        &self,
        house: &SmartHome,
        room: &String,
        device: &String,
    ) -> SmartHomeResult<String> {
        let no_room = format!("В доме <<{}>> нет помещения <<{}>>", house.name, room);
        let no_device = format!("В помещении: <<{}>> нет устройства: <<{}>>", room, device);

        match house.rooms.get(room) {
            Some(room) => match room.get(device) {
                Some(device) => Ok(device.report()),
                None => Err(SmartHomeError::from(no_device)),
            },
            None => Err(SmartHomeError::from(no_room)),
        }
    }
}

pub trait IterableProvider {
    fn as_set(&self) -> HashSet<String>;
}
