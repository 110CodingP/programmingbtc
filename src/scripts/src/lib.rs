mod codes;
pub mod helpers;
mod traits;
mod utils;

use std::fmt::format;

use helpers::Stack;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use rug::Integer;
use sha2::{Digest, Sha256};
use utils::parse_varints;

pub enum ScriptOpError {
    StackEmpty,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Script(Vec<String>);

impl Script {
    fn new(commands: Vec<String>) -> Self {
        Script(commands)
    }

    
    /**
     *  Parses a script command (usually a scriptSig or a pubkeyScript) 
     *  using the instruction set defined.
     */
    pub fn parse(command: &str) -> Option<Self> {
        // Convert the str, mostly a str representation of a hex string
        let command_bytes = &Integer::from_str_radix(command, 16)
            .unwrap()
            .to_digits::<u8>(rug::integer::Order::MsfBe);

        // keep track of bytes of the cammand parsed
        let mut count = 0;
        let mut commands = Vec::new();

        while count < command_bytes.len() {
            let current = command_bytes[count as usize]; // get the current byte
            count += 1;

            match current {
                1..=75 => {
                    // Push the next `current` bytes of data to the commands array
                    let bytes_to_push: String =
                        command_bytes[count as usize..count as usize + current as usize]
                            .iter()
                            .map(|byte: &u8| format!("{:02x}", byte))
                            .collect::<Vec<String>>()
                            .join("");
                    let length = bytes_to_push.len() / 2;
                    commands.push(bytes_to_push);

                    // increment the count by the length of the bytes pushed
                    count += length;
                }
                76 => {
                    count += 1;
                    // the next byte  is the length of the data to push
                    let length = &command_bytes[count as usize];
                    let bytes_to_push = command_bytes
                        [(count as usize + 1)..(count as usize + 1 + *length as usize)]
                        .iter()
                        .map(|byte: &u8| format!("{:02x}", byte))
                        .collect::<Vec<String>>()
                        .join("");

                    // increment the count by the length of the bytes pushed
                    count += (bytes_to_push.len() / 2) + 1;
                    
                    commands.push(bytes_to_push);
                }
                77 => {
                    count += 1;
                    // the next 2 bytes contain the number of bytes to be oushed onto-stack
                    let length = &command_bytes[count as usize..(count as usize + 2)]
                        .iter()
                        .fold(0, |acc, &x| acc + x as u16);

                    let bytes_to_push = command_bytes
                        [(count as usize + 2)..(count as usize + 2 + *length as usize)]
                        .iter()
                        .map(|byte: &u8| format!("{:02x}", byte))
                        .collect::<Vec<String>>()
                        .join("");

                    // increment the count by the length of the bytes pushed
                    count += (bytes_to_push.len() / 2) + 2;

                    commands.push(bytes_to_push);
                }
                78 => {
                    count += 1;
                    // the next 4 bytes contain the number of bytes to be oushed onto-stack
                    let length = &command_bytes[count as usize..(count as usize + 4)]
                        .iter()
                        .fold(0, |acc, &x| acc + x as u32);

                    let bytes_to_push = command_bytes
                        [(count as usize + 4)..(count as usize + 4 + *length as usize)]
                        .iter()
                        .map(|byte: &u8| format!("{:02x}", byte))
                        .collect::<Vec<String>>()
                        .join("");

                    // increment the count by the length of the bytes pushed
                    count += (bytes_to_push.len() / 2) + 4;

                    commands.push(bytes_to_push);
                }
                _ => {
                    // push the current byte to the commands array
                    commands.push(format!("{:02x}", current));
                }
            }
        }

        if count != command_bytes.len() {
            None
        } else {
            Some(Self::new(commands))
        }
    }

    pub fn serialize(&self) -> String {
        let mut result = String::new();

        for cmd in &self.0 {
            let length = cmd.len() / 2;
            match length {
                2..=75 => {
                    result.push_str(&format!("{:02x}", length));
                },
                76..0x100 => {
                    result.push_str("4c");
                    result.push_str(&format!("{:02x}", length));
                },
                0x100..=520 => {
                    result.push_str("4d");
                    result.push_str(&format!("{:04x}", length));
                },
                _ => {
                    println!("Error: Length of command is invalid");
                }
            }
            result.push_str(cmd);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::StackOps;

    use super::*;

    #[test]
    fn test_op_hash160() {
        let mut stack = Stack::new();
        stack.push(String::from("hello world").into_bytes());
        assert!(stack.op_hash160(), "Hashing should succeed");

        assert_eq!(stack.length(), 1, "Stack should have one item");
        let item = stack.pop().unwrap();
        let item_byte = item
            .iter()
            .map(|byte: &u8| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(item_byte, "d7d5ee7824ff93f94c3055af9382c86c68b5ca92");
    }

    #[test]
    fn test_parse_script() {
        let command = "6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937";
        let script = Script::parse(command).unwrap();

        assert_eq!(script.0[0], "6a");
        assert_eq!(
            script.0[1], 
            "304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601".
            to_string()
        );
        assert_eq!(
            script.0[2], 
            "035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937"
                .to_string()
        );

        let serialized = script.serialize();
        assert_eq!(serialized, command);
    }
}
