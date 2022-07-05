use crate::busio::SerialInterface;
use crate::eldecode::EltakoFrame;
use crate::ringbuff::RingBuff;

use log::{debug, info};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

pub struct Bus {
    serial: SerialInterface,

    decoder: Option<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
}

impl Bus {
    pub fn new() -> Self {
        env_logger::init();

        let mut bus = Bus {
            serial: SerialInterface::new("/dev/ttyUSB0".to_string(), 57600, 1),
            decoder: None,
            exit: Arc::new(AtomicBool::new(false)),
        };

        bus.serial.start().expect("Listener already initialized!");

        let data_lock = bus.serial.shared.clone();
        let exit_lock = bus.exit.clone();

        bus.decoder = Some(spawn(move || {
            let mut temp = RingBuff::new(14, 0);
            let mut bytecounter = 0;

            loop {
                if exit_lock.load(Ordering::Relaxed) {
                    break;
                };

                while let Ok(i) = data_lock.lock().unwrap().buff.reduce() {
                    if i == 0xa5 {
                        bytecounter = 0;
                    }
                    bytecounter += 1;
                    temp.append(i);
                }

                if bytecounter >= 14 {
                    let decoded_frame = EltakoFrame::from_vec(&temp.data[0..14]);
                    if let Ok(frame) = decoded_frame {
                        info!("{}", frame.explain());
                    }
                    temp.reset_offset();
                    bytecounter = 0;
                }
                sleep(Duration::from_millis(1));
            }
        }));

        debug!("Decoder thread started!");

        return bus;
    }
}

impl Drop for Bus {
    fn drop(&mut self) {
        let exit = self.exit.clone();

        exit.store(true, Ordering::Relaxed);
        self.decoder
            .take()
            .unwrap()
            .join()
            .expect("Could not join decoder thread!");

        debug!("Decoder thread stopped!");
    }
}
