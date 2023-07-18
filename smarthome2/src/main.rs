#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

// ***** Пример библиотеки "Умный дом" со статическим содержимым

struct SmartRoom {
    room_name: String,
    devices: HashMap<String, SmartDevice>,
}
impl SmartRoom {
    fn new() -> Self {
        Self {
            room_name: String::default(),
            devices: HashMap::new(),
        }
    }
    fn room_devices(&self) -> Vec<&String> {
        let mut str_dev = Vec::new();
        for device in self.devices.keys() {
            str_dev.push(device);
        }
        str_dev
    }
}
struct SmartHouse {
    house_name: String,
    rooms: HashMap<String, SmartRoom>,
}

impl SmartHouse {
    fn new() -> Self {
        Self {
            house_name: String::from("House"),
            rooms: HashMap::new(),
        }
    }

    fn get_rooms(&self) -> Vec<String> {
        //[&str; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        // todo!("список комнат")
        //let mut list_rooms: [&str] =
        for one_room in self.rooms.keys() {
            println!("Room #{}", one_room);
        }
        vec![String::from("Room 1"), String::from("Room 2")]
    }

    fn devices(&self, room: &str) -> [&str; 3] {
        // Размер возвращаемого массива можно выбрать самостоятельно

        if let Some(single_room) = self.rooms.get(room) {
            for str_name in single_room.devices.keys() {
                println!("Имя устройстав {}", str_name);
            }
        } else {
        }
        if room == "Room 1" {
            ["1", "2", "3"]
        } else {
            ["3", "2", "1"]
        }
    }

    fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
        device_info1: &impl DeviceInfoProvider,
    ) -> String {
        let report_rooms = self.get_rooms();
        String::from(report_rooms.join(" - "))
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn device_info(&self, name: &str, room: &str) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
enum SocketState {
    On,
    Off,
}

struct SmartSocket {
    socket_name: String,
    onoff_status: SocketState,
}
struct SmartThermometer {
    temperature: f32,
}

enum SmartDevice {
    Thermomerer(SmartThermometer),
    Socket(SmartSocket),
}
// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, name: &str, room: &str) -> String {
        let room = room.to_owned();
        let name = name.to_owned();
        let result = room + &name;
        result
    }
}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, name: &str, room: &str) -> String {
        String::from("")
    }
}
fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        socket_name: String::from("Телевизор"),
        onoff_status: SocketState::On,
    };
    let socket2 = SmartSocket {
        socket_name: String::from("Лампа"),
        onoff_status: SocketState::Off,
    };
    let thermo = SmartThermometer { temperature: 22.5 };

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
