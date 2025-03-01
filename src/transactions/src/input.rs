use std::fmt::Debug;

use rug::Integer;
use serde::{Deserialize, Serialize};

use crate::utils::TxFetcher;

#[derive(Debug, Default, Deserialize)]
pub struct PrevOutput {
    pub txid: String,
    pub index: u64,
}

impl PrevOutput {
    pub fn new(txid: String, index: u64) -> PrevOutput {
        PrevOutput {
            txid,
            index,
        }
    }

    pub fn parse_index(byte: &[u8]) -> u64 {
        let mut padded = [0u8; 8];
        padded[..byte.len()].copy_from_slice(byte);
        u64::from_le_bytes(padded)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sequence(pub u32);

impl Sequence {
    pub fn new(sequence: u32) -> Sequence {
        Sequence(sequence)
    }

    pub fn from_bytes(byte: &[u8]) -> Sequence {
        let mut padded = [0u8; 4];
        padded[..byte.len()].copy_from_slice(byte);
        Sequence(u32::from_le_bytes(padded))
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct TxIn {
    pub previous_output: PrevOutput,
    pub script_sig: Option<String>,
    pub sequence: Sequence,
}

impl TxIn {
    pub fn new(prev_output: PrevOutput, sig: Option<String>, sequence: Sequence) -> TxIn {
        TxIn {
            previous_output: prev_output,
            script_sig: sig,
            sequence,
        }
    }

    pub fn value(&self, testnet: bool) -> u64 {
        let mut tx_fetcher = TxFetcher::new(testnet);
        let tx = tx_fetcher.fetch(self.previous_output.txid.clone(), false);
        tx.outputs[self.previous_output.index as usize].value
    }

    pub fn parse_from_bytes(bytes: &[u8]) -> Vec<TxIn> {
        let mut txs = vec![];
        let prev_output = PrevOutput {
            txid: hex::encode(&bytes[0..32]),
            index: PrevOutput::parse_index(&bytes[32..36]),
        };

        let script_sig = match bytes[36] {
            0x00 => None,
            _ => Some(hex::encode(&bytes[36..(bytes.len() - 4)])),
        };

        let sequence = Sequence::from_bytes(&bytes[(bytes.len() - 4)..]);

        txs.push(TxIn {
            previous_output: prev_output,
            script_sig,
            sequence,
        });

        txs
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::from("");

        // serialize the prev_tx_index
        let mut prev_id = Integer::from_str_radix(&self.previous_output.txid, 16).unwrap().to_digits::<u8>(rug::integer::Order::MsfLe);
        prev_id.reverse();
        serialized.push_str(
            &prev_id.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
        );

        // serialize the prev_tx_index
        let index = self.previous_output.index as u32;
        serialized.push_str(
            &index.to_le_bytes()
            .iter()
            .map(|byte| format!("{:02x}", (byte << 2) | 0))
            .collect::<String>()
        );

        // serialize the scriptsig
        let scriptsig = self.script_sig.as_ref().unwrap();
        serialized.push_str(
            scriptsig
        );

        // serialize the sequence
        let sequence = self.sequence.0;
        serialized.push_str(
            &sequence.to_le_bytes()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
        );

        serialized
    }
}