use crate::devices::device::{SmartDevice, VecOfDevice};
use crate::providers::provider::{DeviceInfoProvider, IterableProvider};

use crate::error::{SmartHomeError, SmartHomeResult};
use std::collections::{HashMap, HashSet};

pub struct SmartHome {
    pub name: String,
    rooms: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>,
}

impl SmartHome {
    pub fn new(name: String) -> Self {
        SmartHome {
            name,
            rooms: HashMap::new(),
        }
    }

    fn room(&self, name: &str) -> SmartHomeResult<()> {
        match self.rooms.get(name) {
            Some(_) => Ok(()),
            None => Err(SmartHomeError::from(format!("нет помещения: {}", name))),
        }
    }

    fn device(&self, room: &str, name: &str) -> SmartHomeResult<&dyn SmartDevice> {
        self.room(room)?;
        match self.rooms.get(room).unwrap().get(name) {
            Some(device) => Ok(&**device),
            None => Err(SmartHomeError::from(format!(
                "в помещении {} нет устройства: {}",
                room, name
            ))),
        }
    }

    pub fn get_device(&self, room: &str, name: &str) -> SmartHomeResult<&dyn SmartDevice> {
        let device = self.device(room, name)?;
        Ok(device)
    }

    pub fn add_room(&mut self, name: String, devices: VecOfDevice) -> SmartHomeResult<()> {
        if self.room(&name).is_ok() {
            return Err(SmartHomeError::from(
                "Помещение с таким именем уже есть в доме".to_owned(),
            ));
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

    pub fn del_room(&mut self, name: &str) -> SmartHomeResult<()> {
        match self.room(name) {
            Ok(_) => {
                self.rooms.remove(name);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_rooms(&self) -> HashSet<String> {
        self.rooms.keys().cloned().collect()
    }

    pub fn devices(&self, room: &str) -> SmartHomeResult<HashSet<String>> {
        self.room(room)?;
        Ok(self.rooms.get(room).unwrap().keys().cloned().collect())
    }

    pub fn add_device(&mut self, room: &str, device: Box<dyn SmartDevice>) -> SmartHomeResult<()> {
        self.room(room)?;
        let device_name = device.get_name();
        println!("{}", device_name);
        match self.device(room, &device_name) {
            Ok(_) => Err(SmartHomeError::from(
                "Устройство с таким именем уже есть в помещении".to_owned(),
            )),
            Err(_) => {
                self.rooms
                    .get_mut(room)
                    .unwrap()
                    .insert(device_name, device);
                Ok(())
            }
        }
    }

    pub fn del_device(&mut self, room: &str, device: &str) -> SmartHomeResult<()> {
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

            let room_devices = self.devices(room.as_str()).unwrap();

            for device in room_devices.iter() {
                if provider_devices.contains(&device.clone()) {
                    report += "\n";
                    let part = info_provider
                        .get_device_info(self, &room, device)
                        .unwrap_or_else(|e| e.msg);
                    report += format!("--> {}\n", part).as_str();
                }
                provider_devices.remove(device);
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
    use crate::devices::socket::Socket;

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
            assert_eq!(house.devices(&room).unwrap().len() == 0usize, true);
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

    #[test]
    fn test_del_existing_room_is_ok() {
        let mut house = setup();
        let room: String = "Kitchen".into();

        let rooms_before = house.get_rooms().len();
        let res = house.del_room(&room);
        assert!(res.is_ok());
        assert_eq!(house.get_rooms().len(), rooms_before - 1);
    }

    #[test]
    fn test_del_non_existing_room_is_err() {
        let mut house = setup();
        let room: String = "Bathroom".into();

        let rooms_before = house.get_rooms().len();
        let res = house.del_room(&room);
        assert!(res.is_err());
        assert_eq!(house.get_rooms().len(), rooms_before);
    }

    #[test]
    fn test_add_device() {
        let mut house = setup();
        let room: String = "Kitchen".into();

        let socket1 = Socket::new("1");
        let socket1clone = Socket::new("1");

        let device_before = house.devices(&room).unwrap().len();
        let add_device = house.add_device(&room, Box::new(socket1));
        let device_after = house.devices(&room).unwrap().len();
        assert!(add_device.is_ok());
        assert_eq!(device_before, device_after - 1);

        // добавить устройство с тем же именем не получится
        let device_before = house.devices(&room).unwrap().len();
        let twice_device = house.add_device(&room, Box::new(socket1clone));
        assert!(twice_device.is_err());
        let device_after = house.devices(&room).unwrap().len();
        assert_eq!(device_before, device_after);
    }

    #[test]
    fn test_del_device() {
        let mut house = setup();
        let room: String = "Kitchen".into();
        let another_room: String = "Dining".into();

        let socket1 = Socket::new("1");
        let socket2 = Socket::new("2");

        let _ = house.add_device(&room, Box::new(socket1));

        let device_before = house.devices(&room).unwrap().len();

        let del_device = house.del_device(&room, "Smart Socket 1");
        let device_after = house.devices(&room).unwrap().len();

        assert!(del_device.is_ok());
        assert_eq!(device_before, device_after + 1);

        // удалить устройство к-го нет в комнате не получится
        assert_eq!(house.devices(&room).unwrap().len(), 0);
        let _ = house.add_device(&room, Box::new(socket2));
        assert_eq!(house.devices(&room).unwrap().len(), 1);

        let del_device = house.del_device(&room, "Smart Socket 1");
        assert!(del_device.is_err());

        // удалить устройство к-го нет в комнате не получится
        let del_device = house.del_device(&another_room, "Smart Socket 2");
        assert!(del_device.is_err());

        assert_eq!(house.devices(&room).unwrap().len(), 1);
        let _ = house.del_device(&room, "Smart Socket 2");
        assert_eq!(house.devices(&room).unwrap().len(), 0);
    }
}
