mod bus;
mod busio;
mod device;
mod eldecode;
mod ringbuff;

fn main() {
    let mut bus = bus::Bus::new();

    // bus.scan().unwrap();
    loop {
        // bus.ask_status().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
