mod stack;
use stack::Stack;

mod memory;
use memory::Memory;

mod storage;
use storage::Storage;

mod evm;
use evm::EVM;
use  ethereum_types::{U256, H160};

fn main() {
    println!("--------Stack--------");
    let mut stack = Stack::new();
    stack.push(U256::from(1));
    stack.push(U256::from(2));
    stack.push(U256::from(3));
    stack.push(U256::from(4));
    println!("{}", stack.to_string());
    println!("{}", stack.pop());
    println!("{}", stack.pop());

    println!("{}", stack.to_string());

    println!("--------Memory--------");
    let mut mem = Memory::new();
    mem.store(0, &[1, 2, 3, 4]);
    println!("{:#?}", mem.load(0));

    println!("--------Storage--------");
    let mut storage = Storage::new();
    storage.store(1, &[410]);
    println!("{:#?}", storage.load(1)); // cold access
    println!("{:#?}", storage.load(1)); //warm access
    

    // println!("--------EVM--------");
    // let mut evm = EVM::new(H160::from_low_u64_be(0), 100000, 0, vec![1, 2, 3, 4], vec![1, 2, 3, 4]);
    // println!("{:#?}", evm);
}