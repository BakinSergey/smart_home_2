use crate::devices::device::SmartDevice;
use crate::home::SmartHome;
use crate::providers::type_providers_alt::DeviceInfo;

pub trait DeviceInfoProvider {
    /// метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_info(&self, house: &SmartHome, room: &String, device: &String) -> String {
        house.rooms.get(room).unwrap_or_else(|| panic!("В доме <<{}>> нет  помещения <<{}>>", house.name, room))
            .get(device).unwrap_or_else(|| panic!("В помещении: <<{}>> нет устройства: <<{}>>", room, device))
            .report()
    }
}

pub trait DeviceTypeInfoProvider {
    type DeviceType: SmartDevice;

    fn create_stub(&self) -> Self::DeviceType;

    fn get_device_info(&self, house: &SmartHome, room: &String, device: &String) -> Option<String> {
        let device = house.rooms.get(room).unwrap_or_else(|| panic!("В доме <<{}>> нет  помещения <<{}>>", house.name, room))
            .get(device).unwrap_or_else(|| panic!("В помещении: <<{}>> нет устройства: <<{}>>", room, device));

        let stub = self.create_stub();

        if stub.get_type() == device.get_type() {
            Some(device.report())
        } else {
            None
        }
    }
}

pub trait IterableProvider {
    fn as_vec(&self) -> Vec<String>;
}

pub trait DeviceStructInfoProvider {
    fn get_device_info(&self, house: &SmartHome, room: &str, device: &str) -> Option<String>;
}

pub trait DeviceTypeInfoAltProvider {
    fn get_device_info(&self, house: &SmartHome, room: &str, device: &str) -> Option<String>;
}

impl<T: SmartDevice> DeviceTypeInfoAltProvider for DeviceInfo<T>
{
    fn get_device_info(&self, house: &SmartHome, room: &str, device: &str) -> Option<String> {
        let device = house.rooms.get(room).unwrap_or_else(|| panic!("В доме <<{}>> нет  помещения <<{}>>", house.name, room))
            .get(device).unwrap_or_else(|| panic!("В помещении: <<{}>> нет устройства: <<{}>>", room, device));

        if self.device_stub.get_type() == device.get_type() {
            Some(device.report())
        } else {
            None
        }
    }
}