use crate::busio::SerialInterface;
use crate::device::Device;
use crate::eldecode::EltakoFrame;
use std::path::Path;
use std::vec::Vec;

pub struct Bus {
    serial: SerialInterface,
    device_list: Vec<Device>,
}

impl Bus {
    pub fn new() -> Self {
        env_logger::init();

        let mut bus = Bus {
            serial: SerialInterface::new(
                Path::new("/dev/ttyUSB0"),
                nix::sys::termios::BaudRate::B57600,
            )
            .unwrap(),
            device_list: Vec::new(),
        };

        bus.serial.start().expect("Listener already initialized!");
        return bus;
    }

    pub fn scan(&mut self) -> Result<(), ()> {
        // Some magic before the scan
        let frame = EltakoFrame {
            length: 0xe,
            rorg: 0xf0,
            data: 0x03028708,
            source: 0x04065200,
            status: 0x00,
        };

        if self.serial.write(frame).is_err() {
            return Err(());
        }

        // Do the scan itself
        for i in 1..128 {
            let frame = EltakoFrame {
                length: 0xe,
                rorg: 0xf0,
                data: 0x00000000,
                source: 0x00000000,
                status: i,
            };

            if self.serial.write(frame).is_err() {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn ask_status(&mut self) -> Result<(), ()> {
        for i in self.device_list.clone() {
            let frame = EltakoFrame {
                length: 0xe,
                rorg: 0xfe,
                data: 0x00000000,
                source: 0x00000000,
                status: i.id,
            };

            if self.serial.write(frame).is_err() {
                return Err(());
            }
        }
        Ok(())
    }
}
