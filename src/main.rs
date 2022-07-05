mod bus;
mod busio;
mod eldecode;
mod ringbuff;

fn main() {
    let mut _bus = bus::Bus::new();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
