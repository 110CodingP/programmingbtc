use std::{fmt::{Debug, Display}, rc::Rc};
use input::{PrevOutput, Sequence, TxIn};
use output::TxOut;
use rug::Integer;
use serde::{Deserialize, Serialize};

mod version;
pub mod input;
mod output;
pub mod utils;

use sha2::{Sha256, Digest};
use utils::TxFetcher;
use version::Version;

#[derive(Debug)]
pub enum TransactionError {
    FailedToDecodeTX,
}

/// We construct a Transaction
#[derive(Debug, Default, Deserialize)]
pub struct Transaction {
    version: Version,
    pub inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    locktime: u32,
    pub testnet: bool,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut inputs = String::from("");
        for input in &self.inputs {
            inputs.push_str(&format!("{:?}, ", input));
        }

        let mut outputs = String::from("");
        for output in &self.outputs {
            outputs.push_str(&format!("{:?}, ", output));
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
        self.hash()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    // create a hash of the transaction
    pub fn hash(&self) -> Vec<u8> {
        let serialized = self.serialize();
        
        // setup the hasher
        let mut hasher = Sha256::new();

        // hash the serialized transaction
        hasher.update(serialized);

        // return the hash
        let hash = hasher.finalize();
        hash.to_vec()
    }

    pub fn version(&self) -> Version {
        self.version.clone()
    }

    pub fn fee(&self) -> u64 {
        let mut input_value = 0;    // will hold the accumulation of all inputs
        let mut output_value = 0;   // will hold the accumulation of all outputs

        for input in &self.inputs {
            input_value += input.value(self.testnet);
        }
        
        for output in &self.outputs {
            output_value += output.value;
        }

        input_value - output_value
    }

    pub fn serialize(&self) -> String {
        let mut serialized_tx = String::from("");

        // serialize the version
        let version = self.version.parse();
        serialized_tx.push_str(&version);

        // serialize the input length
        let input_count = utils::encode_varints(self.inputs.len() as u64);
        serialized_tx.push_str(&input_count.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
        // serialize the tx_inputs
        for input in &self.inputs {
            let serialized_inputs = input.serialize();
            serialized_tx.push_str(&serialized_inputs);
        }

        // serialize the output length
        let output_count = utils::encode_varints(self.outputs.len() as u64);
        serialized_tx.push_str(&output_count.iter().map(|byte| format!("{:02x}", byte)).collect::<String>());
        
        // serialize the transaction outputs
        for output in &self.outputs {
            let serialized_outputs = output.serialize();
            serialized_tx.push_str(&serialized_outputs);
        }

        // serialize the locktime
        let locktime = self.locktime.to_le_bytes().iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        serialized_tx.push_str(&locktime);

        serialized_tx
    }

    pub fn parse(raw: &str, testnet: bool) -> Result<Transaction, TransactionError> {
        let mut data_count = 0;
        // Parse the version from the transactiob, first 4 bytes
        let tx_bytes = Rc::new(Integer::from_str_radix(
            raw, 
            16)
            .unwrap()
            .to_digits::<u8>(rug::integer::Order::MsfBe));

        let version_len = 4;
        let version = Version::from_vec(&tx_bytes[data_count..version_len]);
        data_count += version_len;

        // Estimate the transaction input, the next character after the version
        // First, let's determine the length of the input
        let (input_byte_count, input_count) = utils::parse_varints(&tx_bytes, data_count);
        data_count += input_byte_count;

        let mut transactions = vec![];
        // loop through the available inputs, based on the input count and extract each input
        for _ in 0..input_count {
            let mut tx_data_count = data_count;

            // the input starts with the prev_tx_id: 32 bytes
            let mut prev_tx_id = tx_bytes[tx_data_count..(tx_data_count + 32)].to_vec();
            prev_tx_id.reverse();
            tx_data_count += 32;

            // this is followed by the prev_tx_index: 4 bytes
            let prev_tx_index_bytes = &tx_bytes[(tx_data_count)..(tx_data_count + 4)];
            tx_data_count += 4;


            // decode the variable-length scriptsig
            let (scriptsig_byte_count, scriptsig_length) = utils::parse_varints(&tx_bytes, tx_data_count);
            let scriptsig = &tx_bytes[(tx_data_count)..(tx_data_count + scriptsig_length as usize + 1)];
            tx_data_count += scriptsig_byte_count + scriptsig_length as usize;

            // the sequence will take up 4 bytes
            let sequence = &tx_bytes[(tx_data_count)..(tx_data_count + 4)];
            tx_data_count += 4;

            let previous_output = PrevOutput {
                txid: prev_tx_id.iter().map(|byte| format!("{:02x}", byte)).collect::<String>(),
                index: PrevOutput::parse_index(prev_tx_index_bytes),
            };
            let transaction = TxIn::new(
                previous_output, 
                Some(scriptsig.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()), 
                Sequence::from_bytes(sequence)
            );
            transactions.push(transaction);

            tx_data_count -= data_count;

            data_count += tx_data_count;
        }


        // parse the tx outputs
        let mut outputs = vec![];
        let (_, output_count) = utils::parse_varints(&tx_bytes, data_count);
        for _ in 0..output_count {
            let mut tx_data_count = data_count;

            // The output amount is 8 bytes
            let output_amount = &tx_bytes[(tx_data_count + 1)..(tx_data_count + 9)];
            let value = u64::from_le_bytes([
                output_amount[0],
                output_amount[1],
                output_amount[2],
                output_amount[3],
                output_amount[4],
                output_amount[5],
                output_amount[6],
                output_amount[7],
            ]);
            tx_data_count += 9;

            // The scriptpubkey is variable length, let's decode the length
            let (scriptpubkey_byte_count, scriptpubkey_length) = utils::parse_varints(&tx_bytes, tx_data_count);
            let scriptpubkey = &tx_bytes[(tx_data_count)..(tx_data_count + scriptpubkey_length as usize + scriptpubkey_byte_count)];
            tx_data_count += scriptpubkey_byte_count + scriptpubkey_length as usize;

            let output = TxOut::new(
                value, 
                scriptpubkey.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
            );
            outputs.push(output);
            tx_data_count -= data_count;
            data_count += tx_data_count - 1;
        }

        // decode the locktime: 4 bytes
        let locktime_bytes = &tx_bytes[(data_count + 1)..(data_count + 5)];
        let mut locktime = [0u8; 4];
        locktime[..locktime_bytes.len()].copy_from_slice(locktime_bytes);

        Ok(Transaction {
            version,
            inputs: transactions,
            outputs,
            locktime: u32::from_le_bytes(locktime),
            testnet,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_tx() -> &'static str {
        "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600"
    }

    #[test]
    fn test_parse_version() {
        let transaction = Transaction::parse(raw_tx(), true);
        assert!(transaction.is_ok(), "Transaction parse should succeed");
        assert_eq!(transaction.unwrap().version, Version::new(1));
    }

    #[test]
    fn test_parse_inputs() {        
        let transaction = Transaction::parse(raw_tx(), true);
        assert!(transaction.is_ok(), "Transaction parse should succeed");
        let tx = transaction.unwrap();
        
        assert_eq!(tx.inputs.len(), 1);
        let input = &tx.inputs[0];
        assert_eq!(input.previous_output.txid, "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81");
        assert_eq!(input.previous_output.index, 0);
        assert_eq!(input.script_sig, Some("6b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a".to_string()));
        assert_eq!(input.sequence.0, 0xfffffffe);
    }

    #[test]
    fn test_parse_outputs() {        
        let transaction = Transaction::parse(raw_tx(), true);
        assert!(transaction.is_ok(), "Transaction parse should succeed");

        let transaction = transaction.unwrap();
        assert_eq!(transaction.outputs.len(), 2);
        assert_eq!(transaction.outputs[0].value, 32454049);
        assert_eq!(transaction.outputs[0].script_pubkey, "1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac".to_string());
        
        assert_eq!(transaction.outputs[1].value, 10011545);
        assert_eq!(transaction.outputs[1].script_pubkey, "1976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac".to_string());
    }

    #[test]
    fn test_parse_tx_locktime() {
        let transaction = Transaction::parse(raw_tx(), true);
        assert!(transaction.is_ok(), "Transaction parse should succeed");

        let transaction = transaction.unwrap();
        assert_eq!(transaction.locktime, 410393);
    }

    #[test]
    fn test_tx_fee() {
        let transaction = Transaction::parse(raw_tx(), false);
        assert!(transaction.is_ok(), "Transaction parse should succeed");

        let tx = transaction.unwrap();
        let fee = tx.fee();
        assert_eq!(fee, 40000);

        let raw_tx = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";
        let tx = Transaction::parse(raw_tx, false);

        assert!(tx.is_ok(), "Transaction parse should succeed");
        let tx = tx.unwrap();
        let fee = tx.fee();
        assert_eq!(fee, 140500);

    }
}