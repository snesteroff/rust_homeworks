use std::collections::HashMap;
// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

struct SmartRoom {
    room_name: String,
    devices: HashMap<String, SmartDevice>,
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

    fn get_rooms(&self) -> [&str; 2] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        // todo!("список комнат")
        //let mut list_rooms: [&str] =
        ["Room 1", "Room 2"]
    }

    fn devices(&self, room: &str) -> [&str; 3] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        // todo!("список устройств в комнате `room`")
        if room == "Room 1" {
            ["1", "2", "3"]
        } else {
            ["3", "2", "1"]
        }
    }

    fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
        // device_info: DeviceInfoProvider,
    ) -> String {
        let report_rooms = self.get_rooms();

        todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn device_info(name: &str, room: &str);
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
    fn device_info(name: &str, room: &str) {}
}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(name: &str, room: &str) {}
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
    let report1 = house.create_report(/* &info_provider_1 */);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(/* &info_provider_2 */);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
