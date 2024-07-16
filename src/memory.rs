use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory out of bounds")]
    ErrOutOfBounds
}

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>, // 1 byte = 8 bits
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn access(&mut self, offset: usize, size: usize) -> Result<&[u8], MemoryError> {
        if self.len() < offset + size {
            let mut n_mem = vec![0x00; offset + size];
            n_mem[..self.len()].copy_from_slice(&self.data);
            self.data = n_mem;
            return Ok(&self.data[offset..offset + size]);
        }

        Ok(&self.data[offset..offset + size])
    }

    pub fn load(&mut self, offset: usize) -> Result<&[u8], MemoryError> {
        self.access(offset, 32)
    }

    pub fn store(&mut self, offset: usize, value: &[u8]) -> Result<usize, MemoryError> {
        let expansion_cost;
        if self.len() <= offset + value.len() {
            let mut expansion_size = 0;

            if self.len() == 0 {
                expansion_size = 32;
                self.data = vec![0x00; 32]; //initialize memory with 32 zeros if it is empty
            }

            if self.len() < offset + value.len() { // extend more memory if needed
                expansion_size = offset + value.len() - self.len();
                let mut n_mem = vec![0x00; expansion_size];
                n_mem[..self.data.len()].copy_from_slice(&self.data);
                self.data = n_mem;
                expansion_cost = (expansion_size.pow(2) as f64).sqrt() as usize;
            } else {
                expansion_cost = 0;
            }
        } else {
            expansion_cost = 0;
        }
        self.data[offset..offset + value.len()].copy_from_slice(value);
        Ok(expansion_cost)
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn cap(&self) -> usize {
        self.data.capacity()
    }

}