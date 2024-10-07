use crate::devices::device::SmartDevice;
use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;
use crate::providers::provider::{DeviceInfoProvider, IterableProvider};

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a Socket,
    pub thermo: &'b Thermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {}

impl IterableProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn as_vec(&self) -> Vec<String> {
        vec![self.socket.get_name(), self.thermo.get_name(), ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::home::SmartHome;

    #[test]
    #[should_panic]
    fn test_invalid_device_in_room_should_panic_for_bpi() {
        let socket11 = Socket::new("11");
        let thermo1 = Thermometer::new("1");

        let mut house = SmartHome::new(String::from("MyHome"));

        let kitchen = "Kitchen2".to_string();
        let kitchen_devices: Vec<Box<dyn SmartDevice>> = vec![Box::new(socket11.clone())];

        house.add_room(kitchen.clone(), kitchen_devices);

        let info_provider_2 = BorrowingDeviceInfoProvider {
            socket: &socket11,
            thermo: &thermo1,
        };

        let kitchen = "Kitchen2".to_string();
        let smart_hammer = String::from("Smart Hammer");

        info_provider_2.get_device_info(&house, &kitchen, &smart_hammer);
    }

    #[test]
    #[should_panic]
    fn test_invalid_room_in_home_should_panic_for_bpi() {
        let socket11 = Socket::new("11");
        let thermo1 = Thermometer::new("1");

        let mut house = SmartHome::new(String::from("MyHome"));


        let kitchen = "Kitchen".to_string();
        let kitchen_devices: Vec<Box<dyn SmartDevice>> = vec![Box::new(socket11.clone())];

        let socket_11_name = String::from("Smart Socket 11");

        house.add_room(kitchen.clone(), kitchen_devices);

        let info_provider_2 = BorrowingDeviceInfoProvider {
            socket: &socket11,
            thermo: &thermo1,
        };
        let bedroom = String::from("Bedroom");


        info_provider_2.get_device_info(&house, &bedroom, &socket_11_name);
    }
}