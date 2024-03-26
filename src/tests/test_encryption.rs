#[cfg(test)]
mod tests {
    use crate::adapter::interface::RSAEncrytion;
    use crate::libs::{encryption::Encryption};
    use crate::adapter::rsa_encryption::RSA;

    #[test]
    fn test_encryption_message_with_data_passed() {
        let crypto : RSA = RSA::new(2048);
        let encryption : Encryption = Encryption::new(crypto);
        let (private_key,public_key) = encryption.generate_keys();

        let message : String = String::from("Hello World");
        let encrypted_message : Vec<u8> = encryption.encrypt_message(message.clone(), public_key.clone());
        let decrypted_message : String = encryption.decrypt_message(encrypted_message.clone(), private_key.clone());

        assert_eq!(message,decrypted_message);
    }

    #[test]
    fn test_encryption_message_without_data_failed() {
        let crypto : RSA = RSA::new(2048);
        let encryption : Encryption = Encryption::new(crypto);
        let (private_key,public_key) = encryption.generate_keys();

        let message : String = String::from("");
        let encrypted_message : Vec<u8> = encryption.encrypt_message(message.clone(), public_key.clone());
        let decrypted_message : String = encryption.decrypt_message(encrypted_message.clone(), private_key.clone());

        assert_eq!(message,decrypted_message);
    }
}