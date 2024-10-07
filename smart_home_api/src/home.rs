use crate::devices::device::{SmartDevice, VecOfDevice};
use crate::providers::provider::{
    DeviceInfoProvider,
    DeviceTypeInfoProvider,
    DeviceTypeInfoAltProvider,
    IterableProvider};

use std::collections::HashMap;


pub struct SmartHome {
    pub name: String,
    pub rooms: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>,
}

impl SmartHome {
    pub fn new(name: String) -> Self {
        SmartHome { name, rooms: HashMap::new() }
    }

    pub fn add_room(&mut self, name: String, devices: VecOfDevice) {
        self.rooms.insert(name.clone(), HashMap::new());

        let mut room_devices: HashMap<String, Box<dyn SmartDevice>> = HashMap::new();

        for d in devices {
            let dev_name = d.get_name();
            room_devices.insert(dev_name, d);
        }

        self.rooms.insert(name, room_devices);
    }

    pub fn get_rooms(&self) -> Vec<String> {
        let res = self.rooms.keys().cloned().collect();
        res
    }

    pub fn devices(&self, room: &str) -> Vec<String> {
        match self.rooms.get(room) {
            Some(room_devices) => room_devices.keys().cloned().collect(),
            None => Vec::new()
        }
    }

    /// отчет о состоянии всех устройств в доме
    pub fn create_report(
        &self,
    ) -> String {
        let mut report: String = String::from("");

        for room in self.get_rooms() {
            report += "\n";
            report += room.clone().as_str();

            for (_name, device) in self.rooms[&room].iter() {
                report += "\n";
                report += format!("--> {}\n", device.report()).as_str();
            }
        }
        report
    }

    /// отчет о состоянии устройств в разрезе провайдера
    pub fn create_provider_report(
        &self,
        info_provider: &(impl DeviceInfoProvider + IterableProvider),
    ) -> String {
        let mut report: String = String::from("");
        let provider_devices = info_provider.as_vec();

        for room in self.get_rooms() {
            report += "\n";
            report += room.as_str();

            for device in self.devices(room.as_str()) {
                if provider_devices.contains(&device) {
                    report += "\n";
                    let device_report = info_provider.get_device_info(self, &room, &device);
                    report += format!("--> {}\n", device_report).as_str();
                }
            }
        }
        report
    }

    /// отчет о состоянии устройств в разрезе типа устройства
    pub fn create_device_type_provider_report(
        &self,
        info_provider: &impl DeviceTypeInfoProvider,
    ) -> String {
        let mut report: String = String::from("");

        for room in self.get_rooms() {
            report += "\n";
            report += room.as_str();

            for device in self.devices(room.as_str()) {
                if let Some(device_report) = info_provider.get_device_info(self, &room, &device) {
                    report += "\n";
                    report += format!("--> {}\n", device_report).as_str();
                }
            }
        }
        report
    }

    /// отчет о состоянии устройств в разрезе типа устройства
    /// (alt provider type)
    pub fn create_alt_device_type_provider_report(
        &self,
        info_provider: &impl DeviceTypeInfoAltProvider,
    ) -> String {
        let mut report: String = String::from("");

        for room in self.get_rooms() {
            report += "\n";
            report += room.as_str();

            for device in self.devices(room.as_str()) {
                if let Some(device_report) = info_provider.get_device_info(self, &room, &device) {
                    report += "\n";
                    report += format!("--> {}\n", device_report).as_str();
                }
            }
        }
        report
    }


}