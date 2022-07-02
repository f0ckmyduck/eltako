use log::debug;

mod busio;
mod eldecode;
mod ringbuff;

fn main() {
    env_logger::init();

    let mut serial = busio::SerialInterface::new("/dev/ttyUSB0".to_string(), 57600, 10);

    serial.start().expect("Listener already initialized!");

    let data_lock = serial.shared.clone();
    loop {
        {
            let mut data = data_lock.lock().unwrap();

            while let Ok(i) = data.buff.reduce() {
                if i == 0xa5 {
                    debug!("");
                }
                debug!("{:#2x} ", i);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
