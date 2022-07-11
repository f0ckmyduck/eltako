#[derive(Clone, Copy)]
pub struct Device {
    pub id: u8,
}

impl Device {
    pub fn new(id: u8) -> Self {
        Device { id: 0x00 }
    }
}

impl Default for Device {
    fn default() -> Self {
        Device { id: 0x00 }
    }
}
