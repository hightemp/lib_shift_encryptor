#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_and_decryption() {
        let key = String::from("test_pass");
        let data = String::from("Long text with repeated data lll repeated data").into_bytes();
        let mut encrypted_result = Vec::new();
        let mut decrypted_result = Vec::new();

        println!("Key: {:?}", key);
        println!("Original data: {:?}", data);    
        println!("Original data length: {:?}", data.len());    

        println!("Encrypting data...");
        let encrypt_result = lib_shift_encryptor::encrypt(
            key.clone(), 
            data, 
            &mut encrypted_result
        );

        println!("Encryption result code: {:?}", encrypt_result);
        println!("Encrypted data: {:?}", encrypted_result);
        println!("Encrypted data length: {:?}", encrypted_result.len());    

        println!("Decrypting data...");
        let decrypt_result = lib_shift_encryptor::decrypt(
            key.clone(), 
            encrypted_result, 
            &mut decrypted_result
        );

        println!("Decryption result code: {:?}", decrypt_result);
        println!("Decrypted data: {:?}", decrypted_result);
        println!("Decrypted data length: {:?}", decrypted_result.len());    

        let decrypted_string = String::from_utf8(decrypted_result)
            .expect("Failed to convert decrypted data to UTF-8 string");
        println!("Decrypted string: {:?}", decrypted_string);
    }
}
