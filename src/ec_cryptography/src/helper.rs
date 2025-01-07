use sha256::digest;

pub fn double_hash(data: &str) -> Vec<u8> {
    // first hash round
    let mut hash = digest(data.as_bytes());

    // second hash round
    hash = digest(hash);
    println!("hash: {:?}", hash);

    hash.into_bytes()
}