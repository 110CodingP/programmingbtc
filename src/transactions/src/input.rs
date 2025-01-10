use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrevOutput {
    pub txid: String,
    pub index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxIn {
    pub previous_output: PrevOutput,
    script_sig: Option<String>,
    sequence: u32,
}

