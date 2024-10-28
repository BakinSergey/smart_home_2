use crate::devices::device::SmartDevice;
use crate::devices::socket::Socket;
use std::collections::HashSet;

use crate::providers::provider::{DeviceInfoProvider, IterableProvider};

pub struct OwningDeviceInfoProvider {
    pub socket: Socket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {}

impl IterableProvider for OwningDeviceInfoProvider {
    fn as_set(&self) -> HashSet<String> {
        HashSet::<String>::from([self.socket.get_name()])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::home::SmartHome;

    fn setup() -> (SmartHome, OwningDeviceInfoProvider) {
        let socket1 = Socket::new("1");

        let mut house = SmartHome::new(String::from("MyHome"));

        let kitchen = "Kitchen".to_string();
        let kitchen_devices: Vec<Box<dyn SmartDevice>> = vec![Box::new(socket1.clone())];

        house.add_room(kitchen.clone(), kitchen_devices).unwrap();

        let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

        (house, info_provider_1)
    }

    #[test]
    fn test_invalid_device_in_room_return_error_for_opi() {
        let (house, provider) = setup();

        let kitchen = "Kitchen".to_string();
        let smart_hammer = String::from("Smart Hammer");

        let res = provider.get_device_info(&house, &kitchen, &smart_hammer);

        assert!(res.is_err());
        assert!(res.err().unwrap().msg.contains("нет устройства"));
    }

    #[test]
    fn test_invalid_room_in_home_return_error_for_opi() {
        let (house, provider) = setup();

        let socket_1_name = String::from("Smart Socket 1");
        let bedroom = String::from("Bedroom");

        let res = provider.get_device_info(&house, &bedroom, &socket_1_name);

        assert!(res.is_err());
        assert!(res.err().unwrap().to_string().contains("нет помещения"));
    }
}
