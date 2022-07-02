use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct RingBuff<T: Copy> {
    pub data: Vec<T>,
    write_offset: usize,
    read_offset: usize,
    wrap_flag: bool,
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

    pub fn append(&mut self, appendage: T) {
        self.data[self.write_offset] = appendage;

        if self.write_offset < self.data.len() - 1 {
            self.write_offset += 1;
        } else {
            self.write_offset = 0;
            self.wrap_flag = true;
        }
    }

    pub fn reduce(&mut self) -> Result<T, ()> {
        let temp = self.data[self.read_offset];

        if self.read_offset < self.write_offset || self.wrap_flag {
            if self.read_offset < self.data.len() - 1 {
                self.read_offset += 1;
            } else {
                self.wrap_flag = false;
                self.read_offset = 0;
            }
        } else {
            return Err(());
        }

        return Ok(temp.clone());
    }

    pub fn reset_offset(&mut self) {
        self.read_offset = 0;
        self.write_offset = 0;
    }
}
