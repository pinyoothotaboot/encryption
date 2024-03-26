
mod libs;
mod adapter;

use crate::adapter::interface::RSAEncrytion;
use crate::libs::{encryption::Encryption};
use crate::adapter::rsa_encryption::RSA;

fn main() -> std::io::Result<()> {
    
    let crypto : RSA = RSA::new(2048);
    let encryption : Encryption = Encryption::new(crypto);
    let (private_key,public_key) = encryption.generate_keys();
    println!("Public key : {} , Private key : {}",public_key,private_key);

    let message : String = String::from("Hello World");
    let encrypted_message : Vec<u8> = encryption.encrypt_message(message.clone(), public_key.clone());
    let decrypted_message : String = encryption.decrypt_message(encrypted_message.clone(), private_key.clone());
    println!("Original Message  1  : {}",message);
    println!("Encrypted Message 1  : {:?}",encrypted_message);
    println!("Decrypted Message 1  : {}",decrypted_message);

    let public_key2= encryption.generate_public_key(private_key.clone());
    println!("Public key2 : {} , Private key : {}",public_key2,private_key);
    let encrypted_message : Vec<u8> = encryption.encrypt_message(message.clone(), public_key2.clone());
    let decrypted_message : String = encryption.decrypt_message(encrypted_message.clone(), private_key);
    println!("Original Message  2  : {}",message);
    println!("Encrypted Message 2  : {:?}",encrypted_message);
    println!("Decrypted Message 2  : {}",decrypted_message);

    assert_eq!(public_key,public_key2);
    Ok(())
}
