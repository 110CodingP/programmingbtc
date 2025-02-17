use std::collections::HashMap;

use rug::Integer;

use crate::Transaction;

pub fn parse_varints(bytes: &[u8], init_count: usize) -> (usize, u64) {
    let (byte_count, length) = match bytes[init_count] {
        0x00..=0xfc => {
            (1, bytes[init_count] as u64)
        },
        0xfd => {
            // if startsWith 'fd', then the next 2 bytes is the length
            let length = Integer::from_digits(
                &bytes[(init_count + 1_usize)..(init_count + 3_usize)],
                rug::integer::Order::LsfLe
            );
            (3, length.to_u64().unwrap())
        },
        0xfe => {
            let mut padded = [0u8; 8];
            let length = &bytes[(init_count + 1_usize)..(init_count + 5_usize)];
            padded[..length.len()].copy_from_slice(&length);
            (4, u64::from_le_bytes(padded))
        },
        0xff => {
            let length = Integer::from_digits(&bytes[(init_count + 1_usize)..(init_count + 9_usize)], rug::integer::Order::LsfLe);
            (8, length.to_u64().unwrap())
        },
    };

    (byte_count, length)
}

pub struct TxFetcher {
    cache: HashMap<String, Transaction>,
    testnet: bool,
}

impl TxFetcher {
    pub fn new(testnet: bool) -> TxFetcher {
        TxFetcher {
            cache: HashMap::new(),
            testnet,
        }
    }
    pub fn get_url(&self) -> &str {
        if self.testnet {
            "https://blockstream.info/testnet/api/"
        } else {
            "https://blockstream.info/api/"
        }
    }

    pub fn fetch(&mut self, tx_id: String, fresh: bool) {
        if fresh || !self.cache.contains_key(&tx_id) {
            let tx_url = format!("{}tx/{}/hex", self.get_url(), tx_id);
            let response = reqwest::blocking::get(
                tx_url,
            ).unwrap().text().unwrap();

            println!("Fetched transaction: {}", response);
        }
    }
}