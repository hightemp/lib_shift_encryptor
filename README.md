# lib_shift_encryptor

Rust encryption library.

## Build

```console
./build.sh
```

## Test

```console
./test.sh
```

## Usage

```rust
use lib_shift_encryptor::*;

fn main() {
    unsafe {
        let sKey: String = String::from("test_pass");
        let oData: Vec<u8> = String::from("test_pass").as_mut_vec().to_vec();
        let mut oEncryptResult: Vec<u8> = Vec::new();
        let mut oDecryptResult: Vec<u8> = Vec::new();

        println!("sKey = {:?}", sKey);
        println!("oData = {:?}", oData);
        println!("oData.len() = {:?}", oData.len());

        println!("> {:?}", "fnEncrypt");
        let iEncryptResult = lib_shift_encryptor::fnEncrypt(sKey.clone(), oData, &mut oEncryptResult);

        println!("iEncryptResult = {:?}", iEncryptResult);
        println!("oEncryptResult = {:?}", oEncryptResult);
        println!("oEncryptResult.len() = {:?}", oEncryptResult.len());

        println!("> {:?}", "fnDecrypt");
        let iDecryptResult = lib_shift_encryptor::fnDecrypt(sKey.clone(), oEncryptResult, &mut oDecryptResult);

        println!("iDecryptResult = {:?}", iDecryptResult);
        println!("oDecryptResult = {:?}", oDecryptResult);
        println!("oDecryptResult.len() = {:?}", oDecryptResult.len());

        let sResult = String::from_utf8(oDecryptResult).expect("Found invalid UTF-8");
        println!("sResult = {:?}", sResult);
    }
}
```
