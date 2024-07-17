use ethereum_types::H160;
use crate::Storage;
use crate::Memory;
use crate::Stack;

mod opcode_instructions;

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

    pub fn peek(&mut self) -> u8 {
        self.program[self.pc]
    }

    pub fn step_next(&mut self) {
        if self.pc < self.program.len() {
            self.pc += 1;
        } else {
            self.stop_flag = true;
        }
    }


    pub fn run(&mut self) {

        while !self.stop_flag {
            self.step_next();
            let op = self.program[self.pc];
            match op {
                STOP => opcode_instructions::stop(self),
                ADD => opcode_instructions::add(self),
                _ => {
                    panic!("Unknown opcode: {}", op);
                    self.stop_flag = true;
                }
            };
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.stack = Stack::new();
        self.memory = Memory::new();
        self.storage = Storage::new();
        self.gas = self.gas;
        self.value = self.value;
        self.call_data = Vec::new();
        self.stop_flag = false;
        self.reverse_flag = false;
        self.return_data = Vec::new();
        self.logs = Vec::new();
    }
}