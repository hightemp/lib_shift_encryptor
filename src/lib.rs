// #![feature(log_syntax)]

use lazy_static::lazy_static;
use std::vec::Vec;

const RESULT_ERROR_EMPTY_PASSWORD: i8 = -1;
const RESULT_ERROR_DATA_IS_EMPTY: i8 = -2;
const RESULT_ERROR_WRONG_PASSWORD: i8 = -3;
const RESULT_SUCCESS: i8 = 1;

lazy_static! {
    static ref METHODS: Vec<u8> = vec![0, 4, 1, 4, 2, 4, 3, 4];
    static ref MAGIC_NUMBERS: Vec<u8> = vec![3, 5, 44, 77];    
}

type Result = i8;
type ByteArray = Vec<u8>;

pub fn encrypt(key: String, data: ByteArray, result: &mut ByteArray) -> Result {
    if key.is_empty() {
        return RESULT_ERROR_EMPTY_PASSWORD;
    }
    if data.is_empty() {
        return RESULT_ERROR_DATA_IS_EMPTY;
    }

    let rounds: i8 = 1;
    let key_bytes: ByteArray = key.clone().into_bytes();

    *result = data.clone();
    result.splice(0..0, MAGIC_NUMBERS.iter().cloned());

    for _ in 0..rounds {
        for (_, &key_byte) in key_bytes.iter().enumerate() {
            for (_, &method) in METHODS.iter().enumerate() {
                match method {
                    0 => {
                        byte_shift(result, true, key_byte);
                    },
                    1 => {
                        byte_shift(result, false, key_byte);
                    },
                    2 => {
                        bit_shift(result, true, key_byte as usize);
                    },
                    3 => {
                        bit_shift(result, false, key_byte as usize);
                    },
                    4 => {
                        xor_bytes(result, &key_bytes);
                    },
                    _ => {}
                }
            }
        }
    }

    RESULT_SUCCESS
}

pub fn decrypt(key: String, data: ByteArray, result: &mut ByteArray) -> Result {
    if key.is_empty() {
        return RESULT_ERROR_EMPTY_PASSWORD;
    }
    if data.is_empty() {
        return RESULT_ERROR_DATA_IS_EMPTY;
    }

    let rounds = 1;
    let key_bytes: ByteArray = key.clone().into_bytes();

    *result = data.clone();

    for _ in 0..rounds {
        for key_idx in (0..key_bytes.len()).rev() {
            for method_idx in (0..METHODS.len()).rev() {
                let key_byte = key_bytes[key_idx];

                match METHODS[method_idx] {
                    1 => {
                        byte_shift(result, true, key_byte);
                    },
                    0 => {
                        byte_shift(result, false, key_byte);
                    },
                    3 => {
                        bit_shift(result, true, key_byte as usize);
                    },
                    2 => {
                        bit_shift(result, false, key_byte as usize);                        
                    },
                    4 => {
                        xor_bytes(result, &key_bytes);
                    },
                    _ => {}
                }
            }
        }
    }

    let extracted_numbers = result.drain(0..MAGIC_NUMBERS.len()).collect::<Vec<u8>>();
    
    if extracted_numbers != Vec::from(MAGIC_NUMBERS.as_slice()) {
        return RESULT_ERROR_WRONG_PASSWORD;
    }    

    RESULT_SUCCESS
}

fn xor_bytes(data: &mut ByteArray, key_bytes: &Vec<u8>) {
    for (idx, byte) in data.iter_mut().enumerate() {
        *byte ^= key_bytes[idx % key_bytes.len()];
    }
}

fn rotate_slice(vec: &mut Vec<u8>, rotate_left: bool, mid: usize, start: usize, end: usize) {
    if start >= end || end > vec.len() {
        return;
    }

    let slice = &vec[start..end];
    let mut rotated_slice: Vec<u8> = Vec::from(slice);
    let fmid = mid % rotated_slice.len();
    
    
    if rotate_left {
        rotated_slice.rotate_left(fmid);
    } else {
        rotated_slice.rotate_right(fmid);    
    }

    vec[start..end].copy_from_slice(&rotated_slice);
}

fn byte_shift(data: &mut ByteArray, rotate_left: bool, key_byte: u8) {
    let line_length = (key_byte % 10 + 5) as usize;    

    let lines_count = ((data.len() as f64) / (line_length as f64)).ceil() as usize;
    
    for line_number in 0..lines_count {
        let position = line_number * line_length;
        let mut next_line_position = (line_number + 1) * line_length;

        if next_line_position>lines_count {
            next_line_position = lines_count;
        }

        rotate_slice(data, rotate_left, key_byte as usize, position, next_line_position);
    }
}

fn bit_shift(data: &mut ByteArray, rotate_left: bool, shift: usize) {
    for byte in data.iter_mut() {
        if rotate_left {
            *byte = byte.rotate_left(shift as u32);
        } else {
            *byte = byte.rotate_right(shift as u32);
        }
    }
}