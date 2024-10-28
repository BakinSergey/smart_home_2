extern crate smart_home_api;

use std::collections::HashSet;
use smart_home_api::{devices, home, providers};

use devices::device::{SmartDevice, VecOfDevice};
use devices::kettle::Kettle as SmartKettle;
use devices::socket::Socket as SmartSocket;
use devices::thermometer::Thermometer as SmartThermometer;

use providers::borrowing_providers::BorrowingDeviceInfoProvider;
use providers::owning_providers::OwningDeviceInfoProvider;

use home::SmartHome;

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket::new("1");
    let socket2 = SmartSocket::new("2");

    let mut socket3 = SmartSocket::new("3");
    socket3.set_broken();

    let socket4 = SmartSocket::new("4");

    //will be ==not found==
    let socket5 = SmartSocket::new("5");

    let thermo1 = SmartThermometer::new("1");
    let thermo2 = SmartThermometer::new("2");

    let mut kettle1 = SmartKettle::new("1");
    kettle1.switch_on();

    let kettle2 = SmartKettle::new("2");

    // Инициализация дома
    let mut house = SmartHome::new(String::from("MyHome"));

    // кухня
    let kitchen = "Kitchen".to_string();
    let kitchen_devices: Vec<Box<dyn SmartDevice>> =
        vec![Box::new(socket1.clone()), Box::new(kettle1)];

    house.add_room(kitchen, kitchen_devices).unwrap();

    // гостиная
    let living = "Living".to_string();
    let living_devices: VecOfDevice = vec![Box::new(socket2.clone()), Box::new(thermo1.clone())];

    house.add_room(living, living_devices).unwrap();

    // спальня
    let bedroom = "Bedroom".to_string();
    let bedroom_devices: VecOfDevice = vec![Box::new(socket3), Box::new(thermo2)];

    house.add_room(bedroom, bedroom_devices).unwrap();

    // смарт-кладовка
    let storeroom = "Storeroom".to_string();
    let storeroom_devices: VecOfDevice = vec![Box::new(socket4), Box::new(kettle2)];

    house
        .add_room(storeroom.clone(), storeroom_devices)
        .unwrap();

    // Библиотека позволяет запросить список помещений в доме.
    let _rooms: HashSet<String> = house.get_rooms();

    // Библиотека позволяет получать список устройств в помещении.
    let _room_devices = house.devices(&storeroom);

    // Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.

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

    // Если устройство не найдено в источнике информации,
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
