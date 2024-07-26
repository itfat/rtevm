use std::collections::HashMap;
use ethereum_types::U256;

#[derive(Debug)]
pub struct Transient {
    data: HashMap<i32, Vec<U256>>, //value can be up to 32 bytes
}

impl Transient {
    pub fn new() -> Transient {
        Transient {
            data: HashMap::new()
        }
    }

    pub fn load(&mut self, key: i32) -> Vec<U256> {
        match self.data.get(&key) {
            Some(data) => data.clone(),
            None => vec![U256::zero()],
        }
    }

    pub fn store(&mut self, key: i32, value: &[U256]) {
        self.data.insert(key, value.to_vec());
    }

    pub fn clear(&mut self) {
        self.data = HashMap::new();
    }
}