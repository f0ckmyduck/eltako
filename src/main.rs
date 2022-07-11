use log::info;

mod bus;
mod busio;
mod device;
mod eldecode;
mod ringbuff;

fn main() {
    use std::thread::sleep;
    use std::time::Duration;
    let mut bus = bus::Bus::new(bus::Mode::Master(bus::Master::AckStatus));

    sleep(Duration::from_millis(3000));
    {
        use crate::eldecode::premaid::*;

        bus.serial
            .write(button(0x00001010, true, Positions::BotRight))
            .unwrap();
        info!("Sent on");

        sleep(Duration::from_millis(100));

        bus.serial
            .write(button(0x00001010, false, Positions::Nothing))
            .unwrap();
        info!("Sent off");
    }

    loop {
        // bus.routine().unwrap();
        sleep(Duration::from_millis(1000));
    }
}
