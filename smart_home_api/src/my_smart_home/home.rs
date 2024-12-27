extern crate stp;

use crate::smart_device::device::SmartDevice;

use std::collections::HashMap;
use std::error::Error;

pub struct Home {
    pub name: String,
    pub rooms: HashMap<String, HashMap<String, Box<dyn SmartDevice>>>,
}

impl Home {
    pub fn new(name: String) -> Result<Self, Box<dyn Error>> {
        let home = Home {
            name,
            rooms: HashMap::new(),
        };

        Ok(home)
    }

    pub fn device_path(r: &str, d: &str) -> String {
        format!("{}=>{}", r, d)
    }
}

#[cfg(test)]
mod test {
    use super::Home;
    use crate::my_smart_home::smart_home::SmartHome;
    use crate::smart_device::device::VecOfDevice;
    use crate::smart_device::socket::Socket;
    use std::collections::HashSet;

    fn setup() -> Home {
        let mut home: Home = Home::new(String::from("MyHome")).unwrap();
        let rooms: HashSet<String> =
            HashSet::from(["Kitchen".into(), "Dining".into(), "Living".into()]);
        for room in rooms {
            match home.add_room(String::from(room), VecOfDevice::new()) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        }
        home
    }

    #[test]
    fn test_get_rooms() {
        let home = setup();
        assert_ne!(home.get_rooms().len(), 0);
    }

    #[test]
    fn test_get_devices() {
        let home = setup();
        let rooms = home.get_rooms();

        for room in &rooms {
            assert_eq!(home.get_devices(&room).unwrap().len() == 0usize, true);
        }
    }

    #[test]
    fn test_get_report() {
        let home = setup();
        let rooms = home.get_rooms();
        let report = home.create_report();

        for room in rooms {
            assert!(report.contains(room.as_str()));
        }
    }

    #[test]
    fn test_add_room_with_same_name_return_error() {
        let mut home = setup();
        let room: String = "Kitchen".into();

        let res = home.add_room(room.clone(), VecOfDevice::new());

        assert!(res.is_err());
        assert!(res.err().unwrap().to_string().contains("same name"));

        //но после удаления, можем добавить
        home.rooms.remove(&room.clone());
        assert!(!home.rooms.contains_key(&room.clone()));

        let res = home.add_room(room, VecOfDevice::new());
        assert!(res.is_ok());
    }

    #[test]
    fn test_del_existing_room_is_ok() {
        let mut home = setup();
        let room: String = "Kitchen".into();

        let rooms_before = home.get_rooms().len();
        let res = home.del_room(&room);
        assert!(res.is_ok());
        assert_eq!(home.get_rooms().len(), rooms_before - 1);
    }

    #[test]
    fn test_del_non_existing_room_is_err() {
        let mut home = setup();
        let room: String = "Bathroom".into();

        let rooms_before = home.get_rooms().len();
        let res = home.del_room(&room);
        assert!(res.is_err());
        assert!(res.err().unwrap().to_string().contains("not exist"));

        assert_eq!(home.get_rooms().len(), rooms_before);
    }

    #[test]
    fn test_add_device() {
        let mut home = setup();
        let room: String = "Kitchen".into();

        let socket1 = Socket::new("1");
        let socket1clone = Socket::new("1");

        let device_before = home.get_devices(&room).unwrap().len();
        let add_device = home.add_device(&room, Box::new(socket1));
        let device_after = home.get_devices(&room).unwrap().len();
        assert!(add_device.is_ok());
        assert_eq!(device_before, device_after - 1);

        // добавить устройство с тем же именем не получится
        let device_before = home.get_devices(&room).unwrap().len();
        let twice_device = home.add_device(&room, Box::new(socket1clone));
        assert!(twice_device.is_err());
        assert!(twice_device
            .err()
            .unwrap()
            .to_string()
            .contains("same name"));

        let device_after = home.get_devices(&room).unwrap().len();
        assert_eq!(device_before, device_after);
    }

    #[test]
    fn test_del_device() {
        let mut home = setup();
        let room: String = "Kitchen".into();
        let another_room: String = "Dining".into();

        let socket1 = Socket::new("1");
        let socket2 = Socket::new("2");

        let _ = home.add_device(&room, Box::new(socket1));

        let device_before = home.get_devices(&room).unwrap().len();

        let del_device = home.del_device(&room, "Smart Socket 1");
        let device_after = home.get_devices(&room).unwrap().len();

        assert!(del_device.is_ok());
        assert_eq!(device_before, device_after + 1);

        // удалить устройство к-го нет в комнате не получится
        assert_eq!(home.get_devices(&room).unwrap().len(), 0);
        let _ = home.add_device(&room, Box::new(socket2));
        assert_eq!(home.get_devices(&room).unwrap().len(), 1);

        let del_device = home.del_device(&room, "Smart Socket 1");
        assert!(del_device.is_err());
        assert!(del_device.err().unwrap().to_string().contains("not exist"));

        // удалить устройство к-го нет в комнате не получится
        let del_device = home.del_device(&another_room, "Smart Socket 2");
        assert!(del_device.is_err());
        assert!(del_device.err().unwrap().to_string().contains("not exist"));

        assert_eq!(home.get_devices(&room).unwrap().len(), 1);
        let _ = home.del_device(&room, "Smart Socket 2");
        assert_eq!(home.get_devices(&room).unwrap().len(), 0);
    }
}
