use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct RingBuff<T: Copy> {
    pub data: Vec<T>,
    pub write_offset: usize,
    pub read_offset: usize,
    pub wrap_flag: bool,
}
impl<T: Copy + Debug> RingBuff<T> {
    pub fn new(initial_size: usize, initial_value: T) -> Self {
        let mut temp = Vec::new();

        for _ in 0..initial_size {
            temp.push(initial_value);
        }

        RingBuff {
            data: temp,
            write_offset: 0,
            read_offset: 0,
            wrap_flag: false,
        }
    }

    pub fn append(&mut self, appendage: T) -> Result<(), ()> {
        self.data[self.write_offset] = appendage;

        if self.write_offset < self.data.len() {
            self.write_offset += 1;
        } else {
            self.write_offset = 0;
            self.wrap_flag = true;
        }

        return Ok(());
    }

    pub fn reduce(&mut self) -> Result<T, ()> {
        let temp = self.data[self.read_offset];

        if self.read_offset < self.data.len() && self.read_offset < self.write_offset {
            self.read_offset += 1;
        } else {
            return Err(());
        }

        if self.wrap_flag {
            self.read_offset = 0;
            self.wrap_flag = false;
        }

        return Ok(temp.clone());
    }
}
