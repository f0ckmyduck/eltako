mod busio;
mod ringbuff;

fn main() {
    let mut ctx = busio::SerialInterface::new("/dev/ttyUSB0".to_string(), 57600, 10);

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

            while let Ok(i) = data.buff.reduce() {
                if i == 0xa5 {
                    println!("");
                }
                print!("{:#2x} ", i);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
