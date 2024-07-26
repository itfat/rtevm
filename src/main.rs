mod stack;
use stack::Stack;

mod memory;
use memory::Memory;

mod storage;
use storage::Storage;

mod transient;
use transient::Transient;

mod evm;
use evm::EVM;
use ethereum_types::{U256, H160};

mod opcodes;

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
    // let program = vec![
    //     0x60, 0x0c,  // PUSH1 12
    //     0x60, 0x0d,  // PUSH1 13
    //     0x60, 0x0e,  // PUSH1 14
    //     0x01,        // ADD (push result 27 onto stack)
    //     0x60, 0x00,  // PUSH1 0 (address for MSTORE)
    //     0x52,        // MSTORE (store 27 at address 0 in memory)
    //     0x60, 0x00,  // PUSH1 0 (address for MLOAD)
    //     0x51,        // MLOAD (load value from address 0 in memory)
    //     0x60, 0x01,  // PUSH1 1 (address for SSTORE)
    //     0x55,        // SSTORE (store value at address 1 in storage)
    //     0x60, 0x01,  // PUSH1 1 (address for SLOAD)
    //     0x54,        // SLOAD (load value from address 1 in storage)
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x02,        // MUL
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x04,        // DIV
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x06,        // MOD
    //     0x60, 0x90,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x03,        // SUB
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x0A,        // EXP
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x10,        // LT
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x11,        // GT
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x14,        // EQ
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x15,        // ISZERO
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x16,        // AND
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x17,        // OR
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x18,        // XOR
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x19,        // NOT
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x1B,        // SHL
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x1C,        // SHR
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x1D,        // SAR
    //     0x60, 0x09,  // PUSH1 0
    //     0x60, 0x12,  // PUSH1 1
    //     0x1E,        // BYTE
    //     0x60, 0x4,  // Byte offset in memory
    //     0x60, 0x0,  // Byte length in memory
    //     0x20,        // KECCAK256
    //     0x60, 0x01,  // PUSH1 0
    //     0x52,
    //     0xA0,

    //     0x00         // STOP
    // ];

    // let program = vec![0x60, 0x0c, 0x60, 0x00, 0x52, 0x60, 0x0d, 0x60, 0x01, 0x52, 0x60, 0xFF, 0x60,0x02, 0x60, 0x00, 0xA1, 0x00 ];

    // let program = vec![
    //     0x60, 0x0c, // PUSH1 0x0c
    //     0x61, 0x0d, 0x0e, // PUSH2 0x0d0e
    //     0x62, 0x00, 0xff, 0x00, // PUSH3 0x00ff00
    //     0x63, 0x11, 0x22, 0x33, 0x44, // PUSH4 0x11223344
    //     0x64, 0x55, 0x66, 0x77, 0x88, 0x99, // PUSH5 0x55667788
    //     0x00 // STOP
    // ];

//     let program = vec![
//     0x60, 0x01, // PUSH1 0x01
//     0x60, 0x02, // PUSH1 0x02
//     0x60, 0x03, // PUSH1 0x03
//     0x60, 0x04, // PUSH1 0x04
//     0x60, 0x05, // PUSH1 0x05
//     0x8A,       // SWAP4
//     0x00        // STOP
// ];

    // let program = vec![
    //     0x60, 0x01, // PUSH1 0x01
    //     0x60, 0x02, // PUSH1 0x02
    //     0x60, 0x03, // PUSH1 0x03
    //     0x60, 0x04, // PUSH1 0x04
    //     0x60, 0x05, // PUSH1 0x05
    //     0x60, 0x06, // PUSH1 0x06
    //     0x60, 0x07, // PUSH1 0x07
    //     0x8B,       // SWAP5
    //     0x00        // STOP
    // ];

    // let program = vec![
    //     0x60, 0x01, // PUSH1 0x01
    //     0x80,       // DUP1
    //     0x00        // STOP  
    // ];

    // DUP3

    // let program = vec![
    //     0x60, 0x01, // PUSH1 0x01
    //     0x60, 0x02, // PUSH1 0x02
    //     0x60, 0x03, // PUSH1 0x03
    //     0x82,       // DUP3
    //     0x00        // STOP
    // ];

    // let program = vec![
    //     0x60, 0x01, // PUSH1 0x01
    //     0x60, 0x02, // PUSH1 0x02
    //     0x60, 0x03, // PUSH1 0x03
    //     0x60, 0x04, // PUSH1 0x04
    //     0x82,       // DUP3
    //     0x00        // STOP
    // ];
//     let program = vec![
//     0x60, 0x04, // PUSH1 0x04 (address of JUMPDEST)
//     0x56,       // JUMP
//     0x01,       // ADD (should be unreachable if jump is correct)
//     0x5B,       // JUMPDEST
//     0x60, 0x01, // PUSH1 0x01
//     0x60, 0x02, // PUSH1 0x02
//     0x60, 0x0c, // PUSH1 0x08 (address of next JUMPDEST)
//     0x57,       // JUMPI
//     0x5B,       // JUMPDEST
//     0x58,       // PC
//     0x50,       // POP
//     0x00        // STOP
// ];

    // let program = vec![
    //     0x60, 0x01, // PUSH1 0x01
    //     0x60, 0x02, // PUSH1 0x02
    //     0x60, 0x03, // PUSH1 0x03
    //     0x50,       // POP
    //     0x00        // STOP
    // ];
    let program = vec![
    0x60, 0x01,       // PUSH1 0x01
    0x60, 0x42,       // PUSH1 0x42
    0x5D,             // TSTORE
    0x60, 0x01,       // PUSH1 0x01
    // 0x5C,             // TLOAD
    0x00              // STOP
];
    let call_data = vec![];
    let sender = H160::zero();  
    let gas = 5000;  
    let value = 0;   

    // Create an instance of EVM
    let mut evm = EVM::new(sender, gas, value, program, call_data);
    println!("Initial EVM state: {:#?}", evm);

    evm.run();

    println!("Final EVM state: {:#?}", evm);
}