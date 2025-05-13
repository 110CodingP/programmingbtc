use ripemd::Ripemd160;
use sha2::{Sha256, Digest};

use crate::{helpers::Stack, traits::StackOps};

pub enum OpCode {
    Op0,
    OpFalse,
    Op1,
    Op16,
    OpDup,
    OpAdd,
    OpCheckSig,
    OpHash256,
    OpHash160
}

impl OpCode {
    pub fn from_u8(code: u8) -> Option<OpCode> {
        match code {
            0x00 => Some(OpCode::Op0),
            0x76 => Some(OpCode::OpDup),
            0x8b => Some(OpCode::OpAdd),
            0xac => Some(OpCode::OpCheckSig),
            0xaa => Some(OpCode::OpHash256),
            0xa9 => Some(OpCode::OpHash160),
            _ => None
        }
    }
}

impl StackOps for Stack<Vec<u8>> {
    fn op_dup(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
    
        let last_item = self.peek();
        self.push(last_item.unwrap().to_vec());
        true
    }

    fn op_hash256(&mut self) -> bool {
        if self.is_empty() {
            return false;
        } 

        let last_item = self.pop().unwrap();
        let mut hasher = Sha256::new();
        hasher.update(last_item.clone().to_vec());
        let result = hasher.finalize();
        
        self.push(result.to_vec());
        true
    }

    fn op_hash160(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        
        // Perform a hash256 first
        let response = self.op_hash256();
    
        match response {
            false => return false,
            true => {
                // perform a ripemd160 hash on the last item in stack
                let last_item = self.pop().unwrap();
                let mut hasher = Ripemd160::new();
                hasher.update(last_item);
                let result = hasher.finalize();
    
                self.push(result.to_vec());
                return true;   
            }
        }
    }

    fn op_checksig(&mut self) -> bool {
        // OP_CHECKSIG will take the last 2 items from stack
        // last item is the pubkey
        // second last item is the signature
        if self.length() < 2 {
            return false;
        }

        let pubkey = self.pop().unwrap();
        let signature = self.pop().unwrap();
        
        true
    }
}
