use thiserror::Error;
use ethereum_types::U256;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory out of bounds")]
    ErrOutOfBounds
}

#[derive(Debug)]
pub struct Memory {
    data: Vec<U256>, // 32 bytes
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }


    // Updated for read
    pub fn access(&self, offset: usize, size: usize) -> Result<&[U256], MemoryError> {
        if self.len() < offset + size {
            return Err(MemoryError::ErrOutOfBounds);
        }

        Ok(&self.data[offset..offset + size])
    }

    pub fn load(&mut self, offset: usize) -> Result<&[U256], MemoryError> {
        self.access(offset, 1)
    }

    pub fn store(&mut self, offset: usize, value: &[U256]) -> Result<usize, MemoryError> {
        let expansion_cost;
        if self.len() <= offset + value.len() {
            let mut expansion_size = 0;

            if self.len() == 0 {
                expansion_size = 32;
                self.data = vec![U256::zero(); 32]; //initialize memory with 32 zeros if it is empty
            }

            if self.len() < offset + value.len() { // extend more memory if needed
                expansion_size = offset + value.len() - self.len();
                let mut n_mem = vec![U256::zero(); expansion_size];
                n_mem[..self.len()].copy_from_slice(&self.data);
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

    pub fn data(&self) -> &[U256] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn cap(&self) -> usize {
        self.data.capacity()
    }

}