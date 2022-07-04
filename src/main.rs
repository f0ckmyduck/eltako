use log::info;
use std::thread::spawn;

mod busio;
mod eldecode;
mod ringbuff;

fn main() {
    env_logger::init();

    let mut serial = busio::SerialInterface::new("/dev/ttyUSB0".to_string(), 57600, 1);

    serial.start().expect("Listener already initialized!");

    let mut bytecounter = 0;
    let data_lock = serial.shared.clone();
    let mut temp = ringbuff::RingBuff::new(14, 0);

    let decoder = spawn(move || loop {
        {
            let mut data = data_lock.lock().unwrap();

            while let Ok(i) = data.buff.reduce() {
                if i == 0xa5 {
                    bytecounter = 0;
                }
                bytecounter += 1;
                temp.append(i);
            }

            if bytecounter >= 14 {
                let decoded_frame = eldecode::EltakoFrame::from_vec(&temp.data[0..14]);
                if let Ok(frame) = decoded_frame {
                    info!("{}", frame.explain());
                }
                temp.reset_offset();
                bytecounter = 0;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    });

    decoder.join().expect("Could not join decoder");
}
