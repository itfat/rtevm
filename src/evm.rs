use ethereum_types::{H160, U256};
use crate::{Storage, Memory, Stack, Transient};
use crate::opcodes::Opcode;

mod opcode_instructions;


#[derive(Debug)]
struct LogEntry {
    topics: Vec<U256>,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct EVM {
    pc: usize,
    stack: Stack,
    memory: Memory,
    storage: Storage,
    transient: Transient,
    sender: H160, //address 20-bytes
    program: Vec<u8>,
    gas: usize,
    value: usize,
    call_data: Vec<u8>,
    stop_flag: bool,
    revert_flag: bool,
    return_data: Vec<u8>,
    logs: Vec<LogEntry>,
}

impl EVM {
    pub fn new(sender: H160, gas: usize, value: usize, program: Vec<u8>, call_data: Vec<u8>) -> Self {
        EVM {
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            transient: Transient::new(),
            sender,
            program,
            gas,
            value,
            call_data,
            stop_flag: false,
            revert_flag: false,
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


    pub fn run(&mut self) {
        while self.continue_execution() {
            let op_u8 = self.fetch_opcode();
            self.execute_opcode(op_u8);
            self.step_next();
        };
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.stack = Stack::new();
        self.memory = Memory::new();
        self.storage = Storage::new();
        self.transient = Transient::new();
        self.gas = self.gas;
        self.value = self.value;
        self.call_data = Vec::new();
        self.stop_flag = false;
        self.revert_flag = false;
        self.return_data = Vec::new();
        self.logs = Vec::new();
    }

    // Helper functions
    fn log(&mut self, topics: Vec<U256>, data: Vec<u8>) {
        let log = LogEntry {
            topics,
            data
        };
        self.logs.push(log);
    }

    fn step_next(&mut self) {
        if self.continue_execution() {
            self.pc += 1;
        } else {
            self.stop_flag = true;
        }
    }

    fn continue_execution(&self) -> bool {
        self.pc < self.program.len() && !self.stop_flag && !self.revert_flag
    }

    fn fetch_opcode(&self) -> u8 {
        self.program[self.pc]
    }

    fn execute_opcode(&mut self, opcode: u8) {
        match Opcode::from_u8(opcode) {
            Opcode::STOP => opcode_instructions::stop(self),
            Opcode::ADD => opcode_instructions::add(self),
            Opcode::PUSH1 => opcode_instructions::push_n(self, 1),
            Opcode::PUSH2 => opcode_instructions::push_n(self, 2),
            Opcode::PUSH3 => opcode_instructions::push_n(self, 3),
            Opcode::PUSH4 => opcode_instructions::push_n(self, 4),
            Opcode::PUSH5 => opcode_instructions::push_n(self, 5),
            Opcode::PUSH6 => opcode_instructions::push_n(self, 6),
            Opcode::PUSH7 => opcode_instructions::push_n(self, 7),
            Opcode::PUSH8 => opcode_instructions::push_n(self, 8),
            Opcode::PUSH9 => opcode_instructions::push_n(self, 9),
            Opcode::PUSH10 => opcode_instructions::push_n(self, 10),
            Opcode::PUSH11 => opcode_instructions::push_n(self, 11),
            Opcode::PUSH12 => opcode_instructions::push_n(self, 12),
            Opcode::PUSH13 => opcode_instructions::push_n(self, 13),
            Opcode::PUSH14 => opcode_instructions::push_n(self, 14),
            Opcode::PUSH15 => opcode_instructions::push_n(self, 15),
            Opcode::PUSH16 => opcode_instructions::push_n(self, 16),
            Opcode::PUSH17 => opcode_instructions::push_n(self, 17),
            Opcode::PUSH18 => opcode_instructions::push_n(self, 18),
            Opcode::PUSH19 => opcode_instructions::push_n(self, 19),
            Opcode::PUSH20 => opcode_instructions::push_n(self, 20),
            Opcode::PUSH21 => opcode_instructions::push_n(self, 21),
            Opcode::PUSH22 => opcode_instructions::push_n(self, 22),
            Opcode::PUSH23 => opcode_instructions::push_n(self, 23),
            Opcode::PUSH24 => opcode_instructions::push_n(self, 24),
            Opcode::PUSH25 => opcode_instructions::push_n(self, 25),
            Opcode::PUSH26 => opcode_instructions::push_n(self, 26),
            Opcode::PUSH27 => opcode_instructions::push_n(self, 27),
            Opcode::PUSH28 => opcode_instructions::push_n(self, 28),
            Opcode::PUSH29 => opcode_instructions::push_n(self, 29),
            Opcode::PUSH30 => opcode_instructions::push_n(self, 30),
            Opcode::PUSH31 => opcode_instructions::push_n(self, 31),
            Opcode::PUSH32 => opcode_instructions::push_n(self, 32),
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
            Opcode::AND => opcode_instructions::and(self),
            Opcode::OR => opcode_instructions::or(self),
            Opcode::XOR => opcode_instructions::xor(self),
            Opcode::NOT => opcode_instructions::not(self),
            Opcode::BYTE => opcode_instructions::byte(self),
            Opcode::SHL => opcode_instructions::shl(self),
            Opcode::SHR => opcode_instructions::shr(self),
            Opcode::SAR => opcode_instructions::sar(self),
            Opcode::KECCAK256 => opcode_instructions::_keccak256(self),
            Opcode::LOG0 => opcode_instructions::log0(self),
            Opcode::LOG1 => opcode_instructions::log1(self),
            Opcode::LOG2 => opcode_instructions::log2(self),
            Opcode::LOG3 => opcode_instructions::log3(self),
            Opcode::LOG4 => opcode_instructions::log4(self),
            Opcode::SWAP1 => opcode_instructions::swap_n(self, 1),
            Opcode::SWAP2 => opcode_instructions::swap_n(self, 2),
            Opcode::SWAP3 => opcode_instructions::swap_n(self, 3),
            Opcode::SWAP4 => opcode_instructions::swap_n(self, 4),
            Opcode::SWAP5 => opcode_instructions::swap_n(self, 5),
            Opcode::SWAP6 => opcode_instructions::swap_n(self, 6),
            Opcode::SWAP7 => opcode_instructions::swap_n(self, 7),
            Opcode::SWAP8 => opcode_instructions::swap_n(self, 8),
            Opcode::SWAP9 => opcode_instructions::swap_n(self, 9),
            Opcode::SWAP10 => opcode_instructions::swap_n(self, 10),
            Opcode::SWAP11 => opcode_instructions::swap_n(self, 11),
            Opcode::SWAP12 => opcode_instructions::swap_n(self, 12),
            Opcode::SWAP13 => opcode_instructions::swap_n(self, 13),
            Opcode::SWAP14 => opcode_instructions::swap_n(self, 14),
            Opcode::SWAP15 => opcode_instructions::swap_n(self, 15),
            Opcode::SWAP16 => opcode_instructions::swap_n(self, 16),
            Opcode::DUP1 => opcode_instructions::dun_n(self, 1),
            Opcode::DUP2 => opcode_instructions::dun_n(self, 2),
            Opcode::DUP3 => opcode_instructions::dun_n(self, 3),
            Opcode::DUP4 => opcode_instructions::dun_n(self, 4),
            Opcode::DUP5 => opcode_instructions::dun_n(self, 5),
            Opcode::DUP6 => opcode_instructions::dun_n(self, 6),
            Opcode::DUP7 => opcode_instructions::dun_n(self, 7),
            Opcode::DUP8 => opcode_instructions::dun_n(self, 8),
            Opcode::DUP9 => opcode_instructions::dun_n(self, 9),
            Opcode::DUP10 => opcode_instructions::dun_n(self, 10),
            Opcode::DUP11 => opcode_instructions::dun_n(self, 11),
            Opcode::DUP12 => opcode_instructions::dun_n(self, 12),
            Opcode::DUP13 => opcode_instructions::dun_n(self, 13),
            Opcode::DUP14 => opcode_instructions::dun_n(self, 14),
            Opcode::DUP15 => opcode_instructions::dun_n(self, 15),
            Opcode::DUP16 => opcode_instructions::dun_n(self, 16),
            Opcode::JUMP => opcode_instructions::jump(self),
            Opcode::JUMPI => opcode_instructions::jumpi(self),
            Opcode::PC => opcode_instructions::pc(self),
            Opcode::JUMPDEST => opcode_instructions::jump_dest(self),
            Opcode::POP => opcode_instructions::pop(self),
            Opcode::TSTORE => opcode_instructions::tstore(self),
            Opcode::TLOAD => opcode_instructions::tload(self),

            Opcode::ADDRESS => opcode_instructions::address(self),
            Opcode::BALANCE => opcode_instructions::balance(self),
            Opcode::ORIGIN => opcode_instructions::origin(self),
            Opcode::CALLER => opcode_instructions::caller(self),
            Opcode::CALLVALUE => opcode_instructions::callvalue(self),
            Opcode::CALLDATALOAD => opcode_instructions::calldataload(self),
            Opcode::CALLDATASIZE => opcode_instructions::calldatasize(self),
            Opcode::CALLDATACOPY => opcode_instructions::calldatacopy(self),
            Opcode::CODESIZE => opcode_instructions::codesize(self),
            Opcode::CODECOPY => opcode_instructions::codecopy(self),
            Opcode::GASPRICE => opcode_instructions::gasprice(self),
            Opcode::EXTCODECOPY => opcode_instructions::extcodecopy(self),
            Opcode::EXTCODESIZE => opcode_instructions::extcodesize(self),
            Opcode::RETURNDATACOPY => opcode_instructions::returndatacopy(self),
            Opcode::RETURNDATASIZE => opcode_instructions::returndatasize(self),
            Opcode::RETURN => opcode_instructions::_return(self),
            Opcode::REVERT => opcode_instructions::revert(self),

            _ => {
                // panic!("Unknown opcode: {:#?}", op_u8);
                // self.stop_flag = true;
            }
        }
    }
}
  