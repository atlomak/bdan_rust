use std::fs;

use sha3::{Digest, Sha3_256};


fn main() {}

#[test]
fn test_hash_of_file() {
    let resource_path = String::from("resources/");
    let data = fs::read(resource_path + "test_file.txt").expect("Missing file");

    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    print!("Hash: {:?}", hex::encode(hash));
}
