mod stack;
use stack::Stack;

fn main() {
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

}