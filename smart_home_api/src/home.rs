use crate::devices::device::{SmartDevice, VecOfDevice};
use crate::providers::provider::{DeviceInfoProvider, IterableProvider};

use crate::error::{SmartHomeError, SmartHomeResult};
use std::collections::{HashMap, HashSet};

pub struct SmartHome {
    pub name: String,
    pub rooms: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>,
}

impl SmartHome {
    pub fn new(name: String) -> Self {
        SmartHome {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: String, devices: VecOfDevice) -> SmartHomeResult<()> {
        if self.rooms.contains_key(&name) {
            return Err(SmartHomeError::from(
                "Помещение с таким именем уже есть в доме".to_owned(),
            ));
        }

        self.rooms.insert(name.clone(), HashMap::new());

        let mut room_devices: HashMap<String, Box<dyn SmartDevice>> = HashMap::new();

        for d in devices {
            let dev_name = d.get_name();
            room_devices.insert(dev_name, d);
        }

        self.rooms.insert(name, room_devices);
        Ok(())
    }

    pub fn get_rooms(&self) -> HashSet<String> {
        self.rooms.keys().cloned().collect()
    }

    pub fn devices(&self, room: &str) -> HashSet<String> {
        match self.rooms.get(room) {
            Some(room_devices) => room_devices.keys().cloned().collect(),
            None => HashSet::new(),
        }
    }

    /// отчет о состоянии всех устройств в доме
    pub fn create_report(&self) -> String {
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
        let mut provider_devices = info_provider.as_set().clone();

        for room in self.get_rooms() {
            report += "\n";
            report += room.as_str();

            for device in self.devices(room.as_str()) {
                if provider_devices.contains(&device.clone()) {
                    report += "\n";
                    let part = info_provider
                        .get_device_info(self, &room, &device)
                        .unwrap_or_else(|e| e.msg);
                    report += format!("--> {}\n", part).as_str();
                }
                provider_devices.remove(&device);
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

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> SmartHome {
        let mut house = SmartHome::new(String::from("MyHome"));
        let rooms: HashSet<String> =
            HashSet::from(["Kitchen".into(), "Dining".into(), "Living".into()]);
        for room in rooms {
            match house.add_room(String::from(room), VecOfDevice::new()) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        }
        house
    }

    #[test]
    fn test_get_rooms() {
        let house = setup();
        assert_ne!(house.get_rooms().len(), 0);
    }

    #[test]
    fn test_get_devices() {
        let house = setup();
        let rooms = house.get_rooms();

        for room in &rooms {
            assert_eq!(house.devices(&room).len() == 0usize, true);
        }
    }

    #[test]
    fn test_get_report() {
        let house = setup();
        let rooms = house.get_rooms();
        let report = house.create_report();

        for room in rooms {
            assert!(report.contains(room.as_str()));
        }
    }

    #[test]
    fn test_add_room_with_same_name_return_error() {
        let mut house = setup();
        let room: String = "Kitchen".into();

        let res = house.add_room(room.clone(), VecOfDevice::new());

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("с таким именем уже есть"));

        //но после удаления, можем добавить
        house.rooms.remove(&room.clone());
        assert!(!house.rooms.contains_key(&room.clone()));

        let res = house.add_room(room, VecOfDevice::new());
        assert!(res.is_ok());
    }
}
