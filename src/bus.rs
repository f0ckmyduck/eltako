use crate::{busio::SerialInterface, device::Device};
use std::{path::Path, vec::Vec};

pub enum Master {
    /// FAM14 -> BA = 2,3
    Ack,
    /// FAM14 -> BA = 4
    AckStatus,
}

pub enum Mode {
    Address,
    Master(Master),
    Slave,
}

pub struct Bus {
    init: bool,
    op_mode: Mode,
    serial: SerialInterface,
    pub device_list: Vec<Device>,
}

impl Bus {
    pub fn new(op_mode: Mode) -> Self {
        env_logger::init();

        let mut bus = Bus {
            init: false,
            op_mode,
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

    pub fn address_enumeration(&mut self) -> Result<(), ()> {
        use crate::eldecode::premaid;

        // Some magic before the scan (Probably some kind of mode-set).
        if self.serial.write(premaid::scan_start()).is_err() {
            return Err(());
        }

        // Do the scan itself.
        for i in 1..128 {
            if self.serial.write(premaid::scan_members(i)).is_err() {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn routine(&mut self) -> Result<(), ()> {
        use crate::eldecode::premaid;

        // Do the first bus scan which is done at every setting.
        if !self.init {
            self.address_enumeration()?;
            self.init = true;
        }

        // Do the normal operating task based on what mode the bus is in.
        match self.op_mode {
            // Wait for an actor to want a new address.
            Mode::Address => {
                self.address_enumeration()?;
            }
            // Play the master and ask for acknowledge telegrams from all saved actors.
            Mode::Master(Master::Ack) => {
                for i in &self.device_list {
                    if self.serial.write(premaid::acknowledge(i.id)).is_err() {
                        return Err(());
                    }
                }
            }
            // Play the master and ask for status telegrams from all saved actors.
            Mode::Master(Master::AckStatus) => {
                for i in &self.device_list {
                    if self.serial.write(premaid::status(i.id)).is_err() {
                        return Err(());
                    }
                }
            }
            // Passively listen to the traffic on the bus.
            Mode::Slave => {}
        }

        Ok(())
    }
}
