use std::collections::HashMap;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    data: HashMap<usize, T>,
    cap: usize,
    size: usize,
    read_offset: Option<usize>,
    write_offset: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: std::fmt::Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: HashMap::new(),
            cap: capacity,
            size: 0,
            read_offset: None,
            write_offset: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn is_full(&self) -> bool {
        self.size() == self.cap
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn grow(&mut self) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.write_offset = match self.write_offset {
            None => Some(0),
            Some(w) => Some(w + 1 % self.cap),
        };
        self.size = self.size + 1;
        Ok(())
    }

    fn force_grow(&mut self) {
        if self.grow() == Ok(()) {
            return;
        }
        self.read_offset = match self.read_offset {
            Some(r) => Some(r + 1 % self.cap),
            None => Some(self.write_offset.unwrap() - 1 % self.cap),
        };
        self.write_offset = Some(self.write_offset.unwrap() + 1 % self.cap);
        self.size = self.size + 1;
    }

    fn advance_read(&mut self) -> Result<(), Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        self.read_offset = match self.read_offset {
            None => Some(0),
            Some(r) => Some(r + 1 % self.cap),
        };
        self.size -= 1;
        Ok(())
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        self.grow()?;
        self.data.insert(self.write_offset.unwrap(), element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.advance_read()?;
        let ret = self.data.remove(&self.read_offset.unwrap()).unwrap();
        Ok(ret)
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
        self.read_offset = None;
        self.write_offset = None;
    }

    pub fn overwrite(&mut self, element: T) {
        self.force_grow();
        self.data.insert(self.write_offset.unwrap(), element);
    }
}
