use super::error::{SmartHomeError, SmartHomeResult};
use super::home::Home;
use crate::info_provider::provider::{DeviceInfoProvider, IterableProvider};
use crate::smart_device::device::{SmartDevice, VecOfDevice};
use std::collections::{HashMap, HashSet};

pub trait SmartHome {
    fn room(&self, name: &str) -> SmartHomeResult<()>;
    fn device(&self, room: &str, name: &str) -> SmartHomeResult<&dyn SmartDevice>;
    fn mut_device(&mut self, room: &str, name: &str) -> SmartHomeResult<&mut dyn SmartDevice>;
    fn get_device(&self, room: &str, name: &str) -> SmartHomeResult<&dyn SmartDevice>;
    fn add_room(&mut self, name: String, devices: VecOfDevice) -> SmartHomeResult<()>;
    fn del_room(&mut self, name: &str) -> SmartHomeResult<()>;
    fn get_rooms(&self) -> HashSet<String>;
    fn get_devices(&self, room: &str) -> SmartHomeResult<HashSet<String>>;
    fn add_device(&mut self, room: &str, device: Box<dyn SmartDevice>) -> SmartHomeResult<()>;
    fn del_device(&mut self, room: &str, device: &str) -> SmartHomeResult<()>;
    fn create_report(&self) -> String;
    fn create_provider_report(
        &self,
        info_provider: &(impl DeviceInfoProvider + IterableProvider),
    ) -> String;
}

impl SmartHome for Home {
    fn room(&self, name: &str) -> SmartHomeResult<()> {
        match self.rooms.get(name) {
            Some(_) => Ok(()),
            None => Err(SmartHomeError::RoomNonExist(name.to_string())),
        }
    }

    fn device(&self, room: &str, name: &str) -> SmartHomeResult<&dyn SmartDevice> {
        self.room(room)?;
        match self.rooms.get(room).unwrap().get(name) {
            Some(device) => Ok(&**device),
            None => Err(SmartHomeError::NoDeviceInRoom {
                name: name.to_string(),
                room: room.to_string(),
            }),
        }
    }

    fn mut_device(&mut self, room: &str, name: &str) -> SmartHomeResult<&mut dyn SmartDevice> {
        self.room(room)?;
        match self.rooms.get_mut(room).unwrap().get_mut(name) {
            Some(device) => Ok(&mut **device),
            None => Err(SmartHomeError::NoDeviceInRoom {
                name: name.to_string(),
                room: room.to_string(),
            }),
        }
    }

    fn get_device(&self, room: &str, name: &str) -> SmartHomeResult<&dyn SmartDevice> {
        let device = self.device(room, name)?;
        Ok(device)
    }

    fn add_room(&mut self, name: String, devices: VecOfDevice) -> SmartHomeResult<()> {
        if self.room(&name).is_ok() {
            return Err(SmartHomeError::RoomSameNameExistInHome(self.name.clone()));
        }

        self.rooms.insert(name.clone(), HashMap::new());

        if !devices.is_empty() {
            let mut room_devices: HashMap<String, Box<dyn SmartDevice>> = HashMap::new();

            for d in devices {
                let dev_name = d.get_name();
                room_devices.insert(dev_name, d);
            }

            self.rooms.insert(name, room_devices);
        }

        Ok(())
    }

    fn del_room(&mut self, name: &str) -> SmartHomeResult<()> {
        match self.room(name) {
            Ok(_) => {
                self.rooms.remove(name);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get_rooms(&self) -> HashSet<String> {
        self.rooms.keys().cloned().collect()
    }

    fn get_devices(&self, room: &str) -> SmartHomeResult<HashSet<String>> {
        self.room(room)?;
        Ok(self.rooms.get(room).unwrap().keys().cloned().collect())
    }

    fn add_device(&mut self, room: &str, device: Box<dyn SmartDevice>) -> SmartHomeResult<()> {
        self.room(room)?;
        let device_name = device.get_name();
        match self.device(room, &device_name) {
            Ok(_) => Err(SmartHomeError::DeviceSameNameExistInRoom(room.to_string())),
            Err(_) => {
                self.rooms
                    .get_mut(room)
                    .unwrap()
                    .insert(device_name, device);
                Ok(())
            }
        }
    }

    fn del_device(&mut self, room: &str, device: &str) -> SmartHomeResult<()> {
        self.room(room)?;
        match self.device(room, device) {
            Ok(_) => {
                self.rooms.get_mut(room).unwrap().remove(device);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// отчет о состоянии всех устройств в доме
    fn create_report(&self) -> String {
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
    fn create_provider_report(
        &self,
        info_provider: &(impl DeviceInfoProvider + IterableProvider),
    ) -> String {
        let mut report: String = String::from("");
        let mut provider_devices = info_provider.as_set().clone();
        println!("{:?}", provider_devices);

        for room in self.get_rooms() {
            report += "\n";
            report += room.as_str();

            let room_devices = self.get_devices(room.as_str()).unwrap();

            for device in room_devices.iter() {
                let dev_id = Self::device_path(&room, device);
                println!("{}", dev_id);
                if provider_devices.contains(&dev_id) {
                    report += "\n";
                    let part = info_provider
                        .get_device_info(self, &room, device)
                        .unwrap_or_else(|e| e.to_string());
                    report += format!("--> {}\n", part).as_str();
                }
                provider_devices.remove(&dev_id);
            }
        }
        report += "\n\n";
        report += "==============\n\n";

        for device in provider_devices.iter() {
            report += format!("\x1b[41m{}\x1b[0m не найдено\n", device).as_str();
        }
        report
    }
}
