extern crate smart_home_api;

use smart_home_api::{devices, home, providers};
use std::collections::HashSet;

use devices::device::{SmartDevice, VecOfDevice};
use devices::kettle::Kettle as SmartKettle;
use devices::socket::Socket as SmartSocket;
use devices::thermometer::Thermometer as SmartThermometer;

use providers::borrowing_providers::BorrowingDeviceInfoProvider;
use providers::owning_providers::OwningDeviceInfoProvider;

use home::SmartHome;

fn main() {
    // uc0.2 Устройство имеет уникальное в рамках помещения имя.
    let socket1 = SmartSocket::new("1");
    let socket2 = SmartSocket::new("2");

    let mut socket3 = SmartSocket::new("3");
    socket3.set_broken();

    let socket4 = SmartSocket::new("4");

    //will be ==not found== uc 3.1
    let socket5 = SmartSocket::new("5");

    //will be added to storeroom uc 2.2
    let socket6 = SmartSocket::new("6");

    let thermo1 = SmartThermometer::new("1");
    let thermo2 = SmartThermometer::new("2");

    let mut kettle1 = SmartKettle::new("1");
    kettle1.switch_on();

    let kettle2 = SmartKettle::new("2");

    // uc0.0 Дом имеет название
    let mut house = SmartHome::new(String::from("MyHome"));

    // кухня
    let kitchen = "Kitchen".to_string();
    let kitchen_devices: Vec<Box<dyn SmartDevice>> =
        vec![Box::new(socket1.clone()), Box::new(kettle1)];

    // uc0.0 Дом содержит несколько помещений.
    house.add_room(kitchen, kitchen_devices).unwrap();

    // гостиная
    // uc0.1 Помещение имеет уникальное название и содержит названия нескольких устройств.
    let living = "Living".to_string();
    let living_devices: VecOfDevice = vec![Box::new(socket2.clone()), Box::new(thermo1.clone())];

    house.add_room(living.clone(), living_devices).unwrap();
    let same_room = house.add_room(living, vec![]);
    assert!(same_room.is_err());
    assert!(same_room.err().unwrap().msg.contains("уже есть"));

    // спальня
    let bedroom = "Bedroom".to_string();
    let bedroom_devices: VecOfDevice = vec![Box::new(socket3), Box::new(thermo2)];

    house.add_room(bedroom, bedroom_devices).unwrap();

    // смарт-кладовка
    let storeroom = "Storeroom".to_string();
    let storeroom_devices: VecOfDevice = vec![Box::new(socket4), Box::new(kettle2)];
    let storeroom_devices_len = storeroom_devices.len();

    house
        .add_room(storeroom.clone(), storeroom_devices)
        .unwrap();

    // Библиотека позволяет:
    // uc 1.1 запросить список помещений в доме.
    let _rooms: HashSet<String> = house.get_rooms();

    // uc 1.2 добавлять помещения.
    let new_room = "NewRoom";
    let added = house.add_room(new_room.to_string(), vec![]);
    assert!(added.is_ok());
    assert_eq!(house.get_rooms().len(), _rooms.len() + 1);

    // uc 1.3 удалять помещения.
    let _rooms: HashSet<String> = house.get_rooms();
    let deleted = house.del_room(new_room);
    assert!(deleted.is_ok());
    assert_eq!(house.get_rooms().len(), _rooms.len() - 1);

    // uc 2.1 Библиотека позволяет получать список устройств в помещении.
    let _room_devices = house.devices(&storeroom);
    assert!(_room_devices.is_ok());
    assert_eq!(_room_devices.unwrap().len(), storeroom_devices_len);

    // uc 2.2 Библиотека позволяет добавлять устройство в помещение.
    let devices_before = house.devices(&storeroom).unwrap().len();
    house.add_device(&storeroom, Box::new(socket6)).unwrap();

    let devices_after = house.devices(&storeroom).unwrap().len();
    assert_eq!(devices_before + 1, devices_after);

    //uc 2.2 Устройство с таким же именем нельзя добавить в помещение
    let socket6clone = SmartSocket::new("6");
    let twice_device = house.add_device(&storeroom, Box::new(socket6clone));
    assert!(twice_device.is_err());
    assert!(twice_device.err().unwrap().msg.contains("уже есть"));
    // storeroom_devices: VecOfDevice = vec![Box::new(socket4), Box::new(kettle2)];

    // uc 2.3 Библиотека позволяет удалить устройство в помещении.
    let devices_before = house.devices(&storeroom).unwrap().len();
    let delete = house.del_device(&storeroom, "Smart Socket 6");
    let devices_after = house.devices(&storeroom).unwrap().len();
    assert_eq!(devices_before, devices_after + 1);

    assert!(delete.is_ok());

    // uc 3 Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.

    // Эта функция принимает в качестве аргумента обобщённый тип,
    // позволяющий получить текстовую информацию
    // о состоянии устройства, для включения в отчёт.
    // (В отчет включаются только устройства содержащиеся в провайдере)

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let report1 = house.create_provider_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo1,
    };
    let report2 = house.create_provider_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("====================");
    println!("Report #2: {report2}");

    // Полный отчет о состоянии устройств в доме
    let report3 = house.create_report();
    println!("====================");
    println!("Report #3: {report3}");

    // uc 3.1 Если устройство не найдено в источнике информации,
    // то вместо текста о состоянии вернуть сообщение об ошибке.
    // (см.также модуль smart_home_api::providers::{owning_providers, borrowing_providers}::test)
    let info_provider_3 = BorrowingDeviceInfoProvider {
        socket: &socket5,
        thermo: &thermo1,
    };
    let report4 = house.create_provider_report(&info_provider_3);
    // Выводим отчёты на экран:
    println!("====================");
    println!("Report #4: {report4}");
}
