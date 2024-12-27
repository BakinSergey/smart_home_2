extern crate smart_home_api;

use smart_device::device::{SmartDevice, VecOfDevice};
use smart_device::kettle::Kettle as SmartKettle;
use smart_device::socket::Socket as SmartSocket;
use smart_device::thermometer::Thermometer as SmartThermometer;
use smart_home_api::{my_smart_home, smart_device};
use std::error::Error;

use my_smart_home::home::Home;
use my_smart_home::smart_home::SmartHome;
use my_smart_home::smart_home_tcp::SmartHomePublicApi;

// test SmartHome structure
//
// "MyHome": "127.0.0.1:55432"
//     "kitchen":
//         "Smart Socket 1", state: Off
//         "Smart Kettle 1", state: On
//     "living":
//         "Smart Socket 2", state: Off
//         "Thermometer 1" , state: On
//     "bedroom":
//         "Smart Socket 3", state: Broken
//         "Thermometer 2" , state: On
//     "storeroom":
//         "Smart Socket 4", state: Off
//         "Smart Kettle 2", state: Off

fn init_home() -> Result<Home, Box<dyn Error>> {
    let socket1 = SmartSocket::new("1");
    let socket2 = SmartSocket::new("2");

    let mut socket3 = SmartSocket::new("3");
    socket3.set_broken();

    let socket4 = SmartSocket::new("4");

    let thermo1 = SmartThermometer::new("1");
    let thermo2 = SmartThermometer::new("2");

    let mut kettle1 = SmartKettle::new("1");
    kettle1.switch_on();

    let kettle2 = SmartKettle::new("2");

    let mut home = Home::new(String::from("MyHome"))?;

    let kitchen = "kitchen".to_string();
    let kitchen_devices: Vec<Box<dyn SmartDevice>> =
        vec![Box::new(socket1.clone()), Box::new(kettle1)];

    home.add_room(kitchen, kitchen_devices).unwrap();

    let living = "living".to_string();
    let living_devices: VecOfDevice = vec![Box::new(socket2.clone()), Box::new(thermo1.clone())];

    home.add_room(living.clone(), living_devices).unwrap();
    let same_room = home.add_room(living, vec![]);
    assert!(same_room.is_err());
    assert!(same_room.err().unwrap().to_string().contains("same name"));

    let bedroom = "bedroom".to_string();
    let bedroom_devices: VecOfDevice = vec![Box::new(socket3), Box::new(thermo2)];

    home.add_room(bedroom, bedroom_devices).unwrap();

    let storeroom = "storeroom".to_string();
    let storeroom_devices: VecOfDevice = vec![Box::new(socket4), Box::new(kettle2)];

    home.add_room(storeroom.clone(), storeroom_devices).unwrap();

    Ok(home)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut home: Home = init_home()?;

    home.serve_public()?;

    Ok(())
}
