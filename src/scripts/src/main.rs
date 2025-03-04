use scripts::helpers::Stack;
use scripts::{op_dup, op_hash160, op_hash256};

mod helpers;

fn main() {
    let mut stack = Stack::new();
    stack.push(String::from("hello world").into_bytes());
    op_hash160(&mut stack);
    
    let item = stack.pop().unwrap();
    let bytes = item.iter().map(|byte: &u8| format!("{:02x}", byte)).collect::<Vec<String>>().join("");
    println!("The item is {:?}", bytes);
}
