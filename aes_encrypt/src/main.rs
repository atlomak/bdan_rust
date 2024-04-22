use std::{
    fs::{self, File},
    io::Write,
};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes128Gcm, Key, Nonce,
};

#[derive(Debug)]
struct CipherData {
    cipher_text: Vec<u8>,
    key: Vec<u8>,
    nonce: Vec<u8>,
}

fn main() {}

fn encrypt(plaintext: &str) -> CipherData {
    let key = Aes128Gcm::generate_key(OsRng);
    let key = Key::<Aes128Gcm>::from_slice(&key);
    print!("Key: {:?}\n", key);

    let cipher = Aes128Gcm::new(&key);
    let nonce = Aes128Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .expect("Error while encrypting data");

    CipherData {
        cipher_text: ciphertext,
        key: key.to_vec(),
        nonce: nonce.to_vec(),
    }
}

fn decrypt(encrypted_data: CipherData) -> Vec<u8> {
    let key = Key::<Aes128Gcm>::from_slice(&encrypted_data.key);
    let cipher = Aes128Gcm::new(&key);
    let nonce = Nonce::from_slice(&encrypted_data.nonce);
    let cipher_text = encrypted_data.cipher_text;

    cipher.decrypt(nonce, cipher_text.as_ref()).unwrap()
}

fn save_to_file(cipherdata: &CipherData, resources_path: String) {
    let mut file = File::create(resources_path.clone() + "key.bin").unwrap();
    file.write_all(&cipherdata.key).unwrap();

    let mut file = File::create(resources_path.clone() + "nonce.bin").unwrap();
    file.write_all(&cipherdata.nonce).unwrap();

    let mut file = File::create(resources_path.clone() + "cipher.bin").unwrap();
    file.write_all(&cipherdata.cipher_text).unwrap();
}

fn cipher_from_files(resources_path: String) -> CipherData {
    let cipher_path = resources_path.clone() + "cipher.bin";
    let key_path = resources_path.clone() + "key.bin";
    let nonce_path = resources_path.clone() + "nonce.bin";

    let cipher_text = fs::read(cipher_path).expect("Cant read file with cipher");
    let key = fs::read(key_path).expect("Cant read file with key");
    let nonce = fs::read(nonce_path).expect("Cant read file with nonce");

    CipherData {
        cipher_text,
        key,
        nonce,
    }
}

#[test]
fn test_encrypt() {
    let plaintext = "Hello world";

    print!("Plaintext: {:?}\n", plaintext.as_bytes());

    let cipher_data = encrypt(plaintext);

    let resources_path = String::from("resources/");

    save_to_file(&cipher_data, resources_path);

    print!("{:?}\n", cipher_data);
}

#[test]
fn test_read_and_decrypt() {
    let resources_path = String::from("resources/");
    let encrypted_data = cipher_from_files(resources_path);

    let plaintext = decrypt(encrypted_data);

    assert_eq!(&plaintext, b"Hello world");
}
