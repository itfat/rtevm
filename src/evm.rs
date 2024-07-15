use ethereum_types::H160;
use crate::Storage;
use crate::Memory;
use crate::Stack;

#[derive(Debug)]
pub struct EVM {
    pc: usize,
    stack: Stack,
    memory: Memory,
    storage: Storage,

    sender: H160,
    program: Vec<u8>,
    gas: usize,
    value: usize,

    call_data: Vec<u8>,

    stop_flag: bool,
    reverse_flag: bool,

    return_data: Vec<u8>,
    logs: Vec<u8>,
}

impl EVM {
    pub fn new(sender: H160, gas: usize, value: usize, program: Vec<u8>, call_data: Vec<u8>) -> Self {
        EVM {
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            sender,
            program,
            gas,
            value,
            call_data,
            stop_flag: false,
            reverse_flag: false,
            return_data: Vec::new(),
            logs: Vec::new(),
        }
    }

    pub fn gas_decrease(&mut self, gas: usize) {
        if self.gas < gas {
            panic!("Not enough gas");
        }
        self.gas -= gas;
    }
}