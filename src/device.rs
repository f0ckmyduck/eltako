#[derive(Clone, Copy)]
pub struct Device {
    pub id: u8,
}

impl Device {
    pub fn new() -> Device {
        Device { id: 0x00 }
    }
}
