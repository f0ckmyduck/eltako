use crate::busio::SerialInterface;

pub struct Bus {
    serial: SerialInterface,
}

impl Bus {
    pub fn new() -> Self {
        env_logger::init();

        let mut bus = Bus {
            serial: SerialInterface::new("/dev/ttyUSB0".to_string(), 57600, 100),
        };

        bus.serial.start().expect("Listener already initialized!");
        return bus;
    }

    pub fn scan(self) {
        for i in 0..128 {}
    }
}
