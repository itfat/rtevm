pub struct Stack {
    data: Vec<u8>,
}


const MAX_SIZE: usize = 1024;

const ERR_STACK_OVERFLOW: &str = "Stack overflow";
const ERR_STACK_UNDERFLOW: &str = "Stack underflow";

impl Stack {
    pub fn push(&mut self, value: u8) {
        if self.data.len() >= MAX_SIZE {
            panic!("{}", ERR_STACK_OVERFLOW);
        }
        self.data.push(value);
    }

    pub fn pop(&mut self) -> u8 {
        if self.data.len() == 0 {
            panic!("{}", ERR_STACK_UNDERFLOW);
        }
        self.data.pop().unwrap()
    }

    pub fn peek(&self) -> u8 {
        *self.data.last().unwrap()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.data.len() {
            s.push_str(&format!("{} ", self.data[i]));
        }
        s
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }   
}