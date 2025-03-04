pub mod helpers;
mod codes;

use helpers::Stack;
use ripemd::{Ripemd160, Digest as RipemdDigest};
use sha2::{Sha256, Digest};

pub enum ScriptOpError {
    StackEmpty,
}

#[derive(Debug)]
struct Script {
    commands: Vec<u8>,
}

impl Script {
    fn new(commands: Vec<u8>) -> Self {
        Script {
            commands
        }
    }
}

pub fn op_dup(stack: &mut Stack<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let last_item = stack.peek();
    stack.push(last_item.unwrap().to_vec());
    true
}

pub fn op_hash256(stack: &mut Stack<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    } 

    let last_item = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(last_item);
    let result = hasher.finalize();
    
    stack.push(result.to_vec());
    true
}

pub fn op_hash160(stack: &mut Stack<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    
    // Perform a hash256 first
    let response = op_hash256(stack);

    match response {
        false => return false,
        true => {
            // perform a ripemd160 hash on the last item in stack
            let last_item = stack.pop().unwrap();
            let mut hasher = Ripemd160::new();
            hasher.update(last_item);
            let result = hasher.finalize();

            stack.push(result.to_vec());
            return true;   
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_hash160() {
        let mut stack = Stack::new();
        stack.push(String::from("hello world").into_bytes());
        assert!(op_hash160(&mut stack), "Hashing should succeed");

        assert_eq!(stack.length(), 1, "Stack should have one item");
        let item = stack.pop().unwrap();
        let item_byte = item.iter().map(|byte: &u8| format!("{:02x}", byte)).collect::<Vec<String>>().join("");
        assert_eq!(item_byte, "d7d5ee7824ff93f94c3055af9382c86c68b5ca92");
    }
}