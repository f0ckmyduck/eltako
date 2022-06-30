mod busio;
mod ringbuff;

fn main() {
    let mut ctx = busio::SerialInterface::new("/dev/ttyUSB0".to_string(), 57600, 100);

    let data_lock = ctx.shared.clone();

    {
        let mut data = data_lock.lock().unwrap();

        for _ in 0..1000 {
            data.buff.data.push(0);
        }
    }

    ctx.start();

    loop {
        {
            let mut data = data_lock.lock().unwrap();

            println!("{} ", data.buff.reduce().unwrap());
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
