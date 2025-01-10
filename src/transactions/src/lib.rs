use std::fmt::Debug;
use serde::{Deserialize, Serialize};

mod version;
mod input;

use version::Version;

#[derive(Debug)]
pub enum TransactionError {
    FailedToDecodeTX,
}

/// We construct a Transaction
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub version: Version,
    inputs: Vec<String>,
    outputs: Vec<String>,
    locktime: u32,
    testnet: bool,
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut inputs = String::from("");
        for input in &self.inputs {
            inputs.push_str(&format!("{}, ", input));
        }

        let mut outputs = String::from("");
        for output in &self.outputs {
            outputs.push_str(&format!("{}, ", output));
        }
        
        write!(f, 
            "Transaction {{ version: {:?}, inputs: {}, outputs: {}, locktime: {} }}", 
            self.version, inputs, outputs, self.locktime
        )
    }
}

impl Transaction {
    // Create a human readable hex of the transaction hash
    pub fn id(&self) -> String {
        "".to_string()
    }

    // create a hash of the transaction
    pub fn hash(&self) -> Vec<u8> {
        vec![]
    }

    pub fn parse(raw: &str) -> Result<Transaction, TransactionError> {
        let transaction: Result<Transaction, serde_json::Error> = serde_json::from_str(raw);
        println!("The transaction is derived as {:?}", transaction);

        match transaction {
            Ok(tx) => {
                Ok(Transaction {
                    version: tx.version,
                    inputs: vec![],
                    outputs: vec![],
                    locktime: 0,
                    testnet: false,
                })
            },
            Err(e) => {
                println!("Error: {:?}", e);
                Err(TransactionError::FailedToDecodeTX)
            }
        }
    }
}