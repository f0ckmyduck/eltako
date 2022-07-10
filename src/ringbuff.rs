use std::{cmp::PartialEq, fmt::Debug, vec::Vec};

#[derive(PartialEq, Debug)]
pub struct RingBuff<T: Copy + Debug + PartialEq> {
    data: Vec<T>,
    write_offset: usize,
    read_offset: usize,
    wrap_flag: bool,
}
impl<T: Copy + Debug + PartialEq> RingBuff<T> {
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

    pub fn readable(&self) -> usize {
        if self.read_offset < self.write_offset {
            return self.write_offset - self.read_offset;
        }

        if self.wrap_flag && self.read_offset > self.write_offset {
            return self.read_offset - self.write_offset;
        }

        return 0;
    }

    pub fn peek(&self) -> T {
        if self.read_offset < self.data.len() {
            return self.data[self.read_offset];
        }
        return self.data[0];
    }

    pub fn reduce(&mut self) -> Result<T, ()> {
        let temp = self.peek();

        if self.readable() >= 1 {
            if self.read_offset < self.data.len() - 1 {
                self.read_offset += 1;
            } else {
                self.read_offset = 0;
                self.wrap_flag = false;
            }
        } else {
            return Err(());
        }

        return Ok(temp.clone());
    }

    pub fn reduce_search(&mut self, search_term: T) -> Result<(), ()> {
        loop {
            if self.peek() == search_term {
                return Ok(());
            } else {
                match self.reduce() {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(());
                    }
                }
            }
        }
    }

    pub fn reduce_slice(&mut self, size: usize) -> Result<Vec<T>, ()> {
        let mut temp = Vec::with_capacity(size);

        for i in 0..size {
            let current = match self.reduce() {
                Ok(x) => x,
                Err(()) => {
                    let (ret, _) = self.read_offset.overflowing_sub(i);
                    self.read_offset = ret % self.data.capacity();
                    return Err(());
                }
            };

            temp.push(current);
        }

        Ok(temp)
    }

    pub fn get_vec(&self) -> Vec<T> {
        self.data.clone()
    }
}
