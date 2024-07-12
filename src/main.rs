mod stack;
use stack::Stack;

mod memory;
use memory::Memory;

mod storage;
use storage::Storage;

fn main() {
    println!("--------Stack--------");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{}", stack.to_string());
    println!("{}", stack.pop());
    println!("{}", stack.pop());
    stack.push(4);
    stack.push(5);
    println!("{}", stack.peek());
    println!("{:#?}", stack.data());

    println!("--------Memory--------");
    let mut mem = Memory::new();
    mem.store(0, &[1, 2, 3, 4]);
    mem.store(5, &[5, 6, 7, 8, 9, 10]);
    println!("{:#?}", mem.load(0).unwrap());
    let cost = mem.store(15, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]);
    println!("{}", cost.unwrap());
    println!("{:#?}", mem.data());
    println!("{:#?}", mem.access(0, 20).unwrap());

    println!("--------Storage--------");
    let mut storage = Storage::new();
    storage.store(1, &[1, 2, 3, 4]);
    storage.store(2, &[5, 6, 7, 8]);
    storage.store(3, "Hello Ethereum".as_bytes());
    println!("{:#?}", storage.load(1));
    println!("{:#?}", storage.load(2));
    println!("{:#?}", storage.load(3));
}