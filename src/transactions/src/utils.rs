use std::{collections::HashMap, rc::Rc};

use rug::Integer;
use hex::ToHex;

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

pub fn encode_varints(length: u64) -> Vec<u8> {
    if length < 0xfd {
        vec![length as u8]
    } else if length < 0x10000 {
        let mut bytes = vec![0xfd];
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes
    } else if length < 0x100000000 {
        let mut bytes = vec![0xfe];
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes
    } else {
        let mut bytes = vec![0xff];
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes
    }
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

    pub fn fetch(&mut self, tx_id: String, fresh: bool) -> &Transaction {
        if fresh || !self.cache.contains_key(&tx_id) {
            let tx_url = format!("{}tx/{}/hex", self.get_url(), tx_id);
            let response = reqwest::blocking::get(
                tx_url,
            ).unwrap().text().unwrap();
            println!("Response: {:?}", response);

            let mut tx = Transaction::default();

            tx.testnet = self.testnet;
            let raw_bytes = Integer::from_str_radix(response.as_str(), 16).unwrap().to_digits::<u8>(rug::integer::Order::MsfBe);
            if raw_bytes[4] == 0x00 {
                let starter_bytes = &raw_bytes[..4];
                starter_bytes.to_vec().copy_from_slice(&raw_bytes[6..]);
                let bytes = starter_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                tx = Transaction::parse(bytes.as_str(), self.testnet).unwrap();
                tx.locktime = Integer::from_digits(&starter_bytes[(starter_bytes.len() - 4)..], rug::integer::Order::LsfLe).to_u32().unwrap();
            } else {
                tx = Transaction::parse(response.as_str(), self.testnet).unwrap();
            }

            println!("Transaction: {:?}", tx.id());

            // if tx.id() != tx_id.clone() {
            //     panic!("Transaction ID does not match");
            // }

            self.cache.insert(tx_id.clone(), tx);
        }
        
        self.cache.get(&tx_id).unwrap()
    }
}