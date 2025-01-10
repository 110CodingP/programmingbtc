use transactions::Transaction;

fn main() {
    let transaction_string = String::from(
        "{\"version\":1,\"inputs\":[\"1234\"],\"outputs\":[\"5678\"],\"locktime\":0,\"testnet\":false}"
    );
    let transaction = Transaction::parse(&transaction_string);
match transaction {
        Ok(tx) => {
            println!("The transaction version is {:?}", tx.version.parse());
        },
        Err(e) => {
            println!("The transaction failed to parse: {:?}", e);
        }
}
}
