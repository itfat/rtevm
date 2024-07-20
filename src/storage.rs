use std::collections::HashMap;
use ethereum_types::U256;

#[derive(Debug)]
pub struct Storage {
    data: HashMap<i32, Vec<U256>>, //value can be up to 32 bytes
    cache: Vec<i32>,
}


impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: HashMap::new(),
            cache: Vec::new(),
        }
    }

    pub fn load(&mut self, key: i32) -> (bool, Vec<U256>) {
        let warm_access = self.cache.contains(&key); // warm slot means it was accessed before and key's in cache
        if !warm_access {
            self.cache.push(key);
        }
        match self.data.get(&key) {
            Some(data) => (warm_access, data.clone()),
            None => (false, vec![U256::zero()]),
        }
        
    }

    pub fn store(&mut self, key: i32, value: &[U256]) {
        self.data.insert(key, value.to_vec());
    }
}