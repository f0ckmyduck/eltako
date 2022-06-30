use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct RingBuff<T: Copy> {
    pub data: Vec<T>,
    pub write_offset: usize,
    pub read_offset: usize,
}
impl<T: Copy + Debug> RingBuff<T> {
    pub fn new(initial_size: usize) -> Self {
        RingBuff {
            data: Vec::with_capacity(initial_size),
            write_offset: 0,
            read_offset: 0,
        }
    }

    pub fn check_wrap(&self, offset: usize) -> usize {
        if offset + 1 < self.data.len() {
            return offset + 1;
        } else {
            return 0;
        }
    }

    pub fn append(&mut self, appendage: T) -> Result<(), ()> {
        // Check if the ring buffer has to wrap around
        self.data[self.write_offset] = appendage;

        self.write_offset = self.check_wrap(self.write_offset);

        return Ok(());
    }

    pub fn reduce(&mut self) -> Result<T, ()> {
        let temp = self.data[self.read_offset];

        self.read_offset = self.check_wrap(self.read_offset);

        return Ok(temp.clone());
    }
}
