use std::fmt::Debug;
use std::vec::Vec;

#[derive(Debug)]
pub struct RingBuff<T: Copy> {
    data: Vec<T>,
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

    pub fn readable(&self) -> bool {
        if self.read_offset < self.write_offset
            || self.wrap_flag && self.read_offset > self.write_offset
        {
            return true;
        }
        false
    }

    pub fn reduce(&mut self) -> Result<T, ()> {
        let temp = self.data[self.read_offset];

        if self.readable() {
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

    pub fn reduce_slice(&mut self, size: usize) -> Result<Vec<T>, ()> {
        let mut temp = Vec::with_capacity(size);

        for _ in 0..size {
            let current = match self.reduce() {
                Ok(x) => x,
                Err(()) => {
                    return Err(());
                }
            };

            temp.push(current);
        }

        Ok(temp)
    }

    pub fn reset_offset(&mut self) {
        self.read_offset = 0;
        self.write_offset = 0;
    }

    pub fn get_write_offset(&self) -> usize {
        self.write_offset
    }

    pub fn get_read_offset(&self) -> usize {
        self.read_offset
    }

    pub fn set_read_offset(&mut self, offset: usize) {
        self.read_offset = offset;
    }

    pub fn get_vec(&self) -> Vec<T> {
        self.data.clone()
    }
}
