use log::info;

mod bus;
mod busio;
mod device;
mod eldecode;
mod ringbuff;

fn main() {
    let mut bus = bus::Bus::new(bus::Mode::Master(bus::Master::AckStatus));

    loop {
        bus.routine().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
