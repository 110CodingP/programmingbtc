use serde::{Deserialize};

use crate::utils::parse_varints;

#[derive(Default, Debug, Deserialize)]
pub struct TxOut {
    pub value: u64,
    pub script_pubkey: String,
}

impl TxOut {
    pub fn new(value: u64, script_pubkey: String) -> TxOut {
        TxOut {
            value,
            script_pubkey,
        }
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::from("");

        // serialize the amount
        let value = self.value
            .to_le_bytes()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        serialized.push_str(&value);

        // serialize the script_pubkey
        serialized.push_str(&self.script_pubkey);

        serialized
    }

    pub fn parse_from_bytes(bytes: &[u8]) -> Vec<TxOut> {
        let mut txs = vec![];
        let mut i = 0;
        while i < bytes.len() {
            let value = u64::from_le_bytes([
                bytes[i],
                bytes[i + 1],
                bytes[i + 2],
                bytes[i + 3],
                bytes[i + 4],
                bytes[i + 5],
                bytes[i + 6],
                bytes[i + 7],
            ]);
            i += 8;
            let (byte_count, script_pubkey_length) = parse_varints(&bytes, i);
            i += byte_count;
            let script_pubkey = hex::encode(&bytes[i..(i + script_pubkey_length as usize)]);
            i += script_pubkey_length as usize;
            txs.push(TxOut::new(value, script_pubkey));
        }
        txs
    }
}