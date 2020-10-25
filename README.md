# lib_shift_encryptor

Rust encryption library.

## Build

```
./build.sh
```

## Test

```
./test.sh
```


## Usage

```rust
use lib_shift_encryptor::*;

fn main() {
    unsafe {
        let sKey: String = String::from("test_pass");
        let oData: Vec<u8> = String::from("test_pass").as_mut_vec().to_vec();
        let mut oResult: Vec<u8> = Vec::new();

        println!("sKey = {:?}", sKey);
        println!("oData = {:?}", oData);

        let iResult = lib_shift_encryptor::fnEncrypt(sKey, oData, &mut oResult);

        println!("iResult = {:?}", iResult);
        println!("oResult = {:?}", oResult);
    }
}
```