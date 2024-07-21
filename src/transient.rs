struct Transient {
    data: HashMap<i32, Vec<U256>>, //value can be up to 32 bytes
}

impl Transient {
    pub fn new() -> Transient {
        Transient {
            data: HashMap::new()
        }
    }

    pub fn load(&mut self, key: i32) -> (bool, Vec<U256>) {
        match self.data.get(&key) {
            Some(data) => (warm_access, data.clone()),
            None => (false, vec![U256::zero()]),
        }
    }

    pub fn store(&mut self, key: i32, value: &[U256]) {
        self.data.insert(key, value.to_vec());
    }

    pub fn clear(&mut self) {
        self.data = self.new();
    }
}