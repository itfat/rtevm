mod stack;
use stack::Stack;

mod memory;
use memory::Memory;

mod storage;
use storage::Storage;

mod evm;
use evm::EVM;
use ethereum_types::{U256, H160};

mod opcodes;
use opcodes::Opcode;

fn main() {
    // println!("--------Stack--------");
    // let mut stack = Stack::new();
    // stack.push(U256::from(1));
    // stack.push(U256::from(2));
    // stack.push(U256::from(3));
    // stack.push(U256::from(4));
    // println!("{}", stack.to_string());
    // println!("{}", stack.pop());
    // println!("{}", stack.pop());

    // println!("{}", stack.to_string());

    // println!("--------Memory--------");
    // let mut mem = Memory::new();
    // mem.store(0, &[U256::from(1), U256::from(2), U256::from(3)]);
    // println!("{:#?}", mem.load(0));

    // println!("--------Storage--------");
    // let mut storage = Storage::new();
    // storage.store(1, &[U256::from(410)]);
    // println!("{:#?}", storage.load(1)); // cold access
    // println!("{:#?}", storage.load(1)); //warm access
    

    println!("--------EVM--------");
    // let program = vec![0x60, 0x0d, 0x60, 0x0e, 0x01];
    let program = vec![
        0x60, 0x0d,  // PUSH1 13
        0x60, 0x0e,  // PUSH1 14
        0x01,        // ADD
        0x60, 0x0e,  // PUSH1  offset
        0x60, 0x0f,    // PUSH1 value
        // 0x52,        // MSTORE
        // 0x51,
        0x00
    ];

    let call_data = vec![];
    let sender = H160::zero();  
    let gas = 1000;  
    let value = 0;   

    // Create an instance of EVM
    let mut evm = EVM::new(sender, gas, value, program, call_data);
    println!("Initial EVM state: {:#?}", evm);

    evm.run();

    println!("Final EVM state: {:#?}", evm);
}