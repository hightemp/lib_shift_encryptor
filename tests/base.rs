#![allow(non_snake_case)]

// use lib_shift_encryptor::fnEncrypt;

#[cfg(test)]
mod tests {
    #[test]
    fn test_fnEncrypt() {
        unsafe {
            let sKey: String = String::from("test_pass");
            let oData: Vec<u8> = String::from("test_pass").as_mut_vec().to_vec();
            let mut oResult: Vec<u8> = Vec::new();

            println!("sKey = {:?}", sKey);
            println!("oData = {:?}", oData);    

            let iResult = lib_shift_encryptor::fnEncrypt(sKey, oData, &mut oResult);

            println!("iResult = {:?}", iResult);
            println!("oResult = {:?}", oResult);
            // assert_eq!(0);
        }
    }

    #[test]
    fn test_fnDecrypt() {
        // assert_eq!(1);
    }
}