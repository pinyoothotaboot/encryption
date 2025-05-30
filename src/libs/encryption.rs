use crate::adapter::{interface::RSAEncrytion, rsa_encryption::RSA};
use log::error;

pub struct Encryption {
    crypto: RSA,
}

impl Encryption {
    pub fn new(crypto: RSA) -> Self {
        Encryption { crypto }
    }

    pub fn generate_keys(&self) -> (String, String) {
        match self.crypto.private_key() {
            Ok(private_key) => match self.crypto.public_key(private_key) {
                Ok((private_key, public_key)) => (private_key, public_key),
                Err(_e) => {
                    error!("generate_keys() - {}", _e);
                    ("".to_string(), "".to_string())
                }
            },
            Err(_e) => {
                error!("generate_keys() - {}", _e);
                ("".to_string(), "".to_string())
            }
        }
    }

    pub fn generate_public_key(&self, private_key: String) -> String {
        match self.crypto.public_key(private_key) {
            Ok((_private_key, public_key)) => public_key,
            Err(_e) => {
                error!("generate_public_key() - {}", _e);
                "".to_string()
            }
        }
    }

    pub fn encrypt_message(&self, message: String, public_key: String) -> Vec<u8> {
        match self.crypto.encrypt(message, public_key) {
            Ok(encrypted_message) => encrypted_message,
            Err(_e) => {
                error!("encrypt_message() - {}", _e);
                vec![]
            }
        }
    }

    pub fn decrypt_message(&self, encrypted_message: Vec<u8>, private_key: String) -> String {
        match self.crypto.decrypt(encrypted_message, private_key) {
            Ok(message) => message,
            Err(_e) => {
                error!("decrypt_message() - {}", _e);
                "".to_string()
            }
        }
    }
}
