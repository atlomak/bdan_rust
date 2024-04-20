use std::{fs::{read_to_string, File}, io::Write};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng}, Aes128Gcm, Key
};

#[derive(Debug)]
struct CipherData {
    cipher: Vec<u8>,
    key: Vec<u8>,
    nonce: Vec<u8>
}
fn main() {
    let filename = "resources/test-file.txt";

    let plaintext = read_to_string(filename).expect(&filename);

}

fn encrypt(plaintext: &str) -> CipherData {
    let key = Aes128Gcm::generate_key(OsRng);
    let key = Key::<Aes128Gcm>::from_slice(&key);
    print!("Key: {:?}\n", key);

    let cipher = Aes128Gcm::new(&key);
    let nonce = Aes128Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).expect("Error while encrypting data");

    CipherData{
        cipher: ciphertext,
        key: key.to_vec(),
        nonce: nonce.to_vec()
    }
}

fn save_to_file(cipherdata: &CipherData) {
    let mut file = File::create("key.bin").unwrap();
    file.write_all(&cipherdata.key).unwrap();
    let mut file = File::create("nonce.bin").unwrap();
    file.write_all(&cipherdata.nonce).unwrap();
    let mut file = File::create("cipher.bin").unwrap();
    file.write_all(&cipherdata.cipher).unwrap();
}

#[test]
fn test_encrypt() {
    let plaintext = "Hello world";

    print!("Plaintext: {:?}\n", plaintext.as_bytes());

    let cipherData = encrypt(plaintext);

    save_to_file(&cipherData);

    print!("{:?}\n", cipherData);
}