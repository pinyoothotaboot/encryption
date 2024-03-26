use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey}, 
    pkcs8::{LineEnding}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey
};
use log::{error};
use rand::thread_rng;
use crate::adapter::interface::{RSAEncrytion};

pub struct RSA {
    bits : usize
}

impl RSAEncrytion for RSA {
    fn new(bits : usize) -> Self {
        RSA{ bits }
    }

    fn public_key(&self,private_key : String) -> Result<(String,String),&'static str> {
        match RsaPrivateKey::from_pkcs1_pem(private_key.as_str()) {
            Ok(rsa) => {
                let rsa_public_key  = RsaPublicKey::from(&rsa);
                match rsa_public_key.to_pkcs1_pem(LineEnding::CRLF) {
                    Ok(public_key) => {
                        Ok((private_key,public_key))
                    },
                    Err(e) => {
                        error!("Error : {e}");
                        Err("Faile to generate public key!.")
                    }
                }
            },
            Err(e) =>{
                error!("Error : {e}");
                Err("Failed to load private key from PEM!.")
            }
        }
    }

    fn private_key(&self) -> Result<String,&'static str> {
        let mut rng = thread_rng();
        match RsaPrivateKey::new(&mut rng,self.bits) {
            Ok(private_key) => {
                match RsaPrivateKey::to_pkcs1_pem(&private_key, LineEnding::CRLF) {
                    Ok(key) =>{
                        Ok(key.to_string())
                    },
                    Err(e) => {
                        error!("Error : {e}");
                        Err("Failed to print to PEM")
                    }
                }
            },
            Err(e) =>{
                error!("Error : {e}");
                Err("Failed to generate a private key!.")
            }
        }
    }

    fn encrypt(&self,message : String , public_key : String) -> Result<Vec<u8>,&'static str> {
        match RsaPublicKey::from_pkcs1_pem(public_key.as_str()) {
            Ok(rsa) => {
                let mut rng = thread_rng();
                match rsa.encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes()) {
                    Ok(encrypted_messages) => {
                        Ok(encrypted_messages)
                    },
                    Err(e) => {
                        error!("Error : {e}");
                        Err("Failed to encrypt message!.")
                    }
                }
            },
            Err(e) => {
                error!("Error : {e}");
                Err("Failed to load public key PEM")
            }
        }
    }

    fn decrypt(&self,encrypted_message : Vec<u8> , private_key : String) -> Result<String,&'static str> {
        match RsaPrivateKey::from_pkcs1_pem(&private_key.as_str()) {
            Ok(rsa) => {
                match rsa.decrypt(Pkcs1v15Encrypt, &encrypted_message) {
                    Ok(messages) => {
                        match String::from_utf8(messages) {
                            Ok(message) => {
                                Ok(message)
                            },
                            Err(e) => {
                                error!("Error : {e}");
                                Err("Failed to convert bytes to string!.")
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error : {e}");
                        Err("Failed to decrypt!.")
                    }
                }
            },
            Err(e) => {
                error!("Error : {e}");
                Err("Failed to load private key from PEM!.")
            }
        }
    }
}