use std::collections::HashMap;

#[derive(Debug)]
pub struct Storage {
    data: HashMap<i32, Vec<u32>>, //value can be up to 32 bytes
    cache: Vec<i32>,
}


impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: HashMap::new(),
            cache: Vec::new(),
        }
    }

    pub fn load(&mut self, key: i32) -> (bool, Vec<u32>) {
        let warm_access = self.cache.contains(&key); // warm slot means it was accessed before and key's in cache
        if !warm_access {
            self.cache.push(key);
        }
        match self.data.get(&key) {
            Some(data) => (warm_access, data.clone()),
            None => (false, vec![0x00]),
        }
        
    }

    pub fn store(&mut self, key: i32, value: &[u32]) {
        self.data.insert(key, value.to_vec());
    }
}