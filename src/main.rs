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
use ethereum_types::H160;

mod opcodes;
mod helper;
fn main() {
 
    println!("--------EVM--------");
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