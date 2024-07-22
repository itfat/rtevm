use ethereum_types::H160;
use crate::Storage;
use crate::Memory;
use crate::Stack;
use crate::opcodes::Opcode;

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
            let op_u8 = self.program[self.pc];
            println!("{:#?}: {:#?}", self.pc, op_u8);
            match Opcode::from_u8(op_u8) {
                Opcode::STOP => opcode_instructions::stop(self),
                Opcode::ADD => opcode_instructions::add(self),
                Opcode::PUSH1 => opcode_instructions::pushN(self),
                Opcode::MSTORE => opcode_instructions::mstore(self),
                Opcode::MLOAD => opcode_instructions::mload(self),
                Opcode::MSTORE8 => opcode_instructions::mstore8(self),
                Opcode::SLOAD => opcode_instructions::sload(self),
                Opcode::SSTORE => opcode_instructions::sstore(self),
                Opcode::MUL => opcode_instructions::mul(self),
                Opcode::SUB => opcode_instructions::sub(self),
                Opcode::DIV => opcode_instructions::div(self),
                Opcode::SDIV => opcode_instructions::sdiv(self),
                Opcode::MOD => opcode_instructions::_mod(self),
                Opcode::SMOD => opcode_instructions::smod(self),
                Opcode::ADDMOD => opcode_instructions::addmod(self),
                Opcode::MULMOD => opcode_instructions::mulmod(self),
                Opcode::EXP => opcode_instructions::exp(self),
                Opcode::LT => opcode_instructions::lt(self),
                Opcode::GT => opcode_instructions::gt(self),
                Opcode::SLT => opcode_instructions::slt(self),
                Opcode::SGT => opcode_instructions::sgt(self),
                Opcode::EQ => opcode_instructions::eq(self),
                Opcode::ISZERO => opcode_instructions::iszero(self),
                _ => {
                    // panic!("Unknown opcode: {:#?}", op_u8);
                    // self.stop_flag = true;
                }
            };
            self.step_next();
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