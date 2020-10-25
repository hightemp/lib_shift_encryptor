#![feature(log_syntax)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

// #![feature(trace_macros)]
// trace_macros!(true);

use std::vec::Vec;

pub const RESULT_ERROR_WRONG_PASSWORD: i8 = -1;
pub const RESULT_ERROR_DATA_IS_EMPTY: i8 = -2;
pub const RESULT_SUCCESS: i8 = -1;

type TResult = i8;

pub fn fnEncrypt(sKey: String, oData: Vec<u8>, oResult: &mut Vec<u8>) -> TResult
{
    if sKey.len() == 0 {
        return RESULT_ERROR_WRONG_PASSWORD;
    }
    if oData.len() == 0 {
        return RESULT_ERROR_DATA_IS_EMPTY;
    }

    unsafe {
        let iRounds: i8 = 1;
        let oKeyByteArray: Vec<u8> = sKey.clone().as_mut_vec().to_vec();

        *oResult = oData.clone();

        oResult.splice(0..0, oKeyByteArray.iter().cloned());

        for iRoundIndex in 0..iRounds {
            for iKeyIndex in 0..oKeyByteArray.len() {
                let aiMethods = vec![0, 4, 1, 4, 2, 4, 3, 4];

                for iMethodIndex in 0..aiMethods.len() {
                    let iLineLength: usize = (oKeyByteArray[iKeyIndex] % 10 + 5) as usize;
                    let mut cByte: u8;

                    match aiMethods[iMethodIndex] {
                        0 => {
                            let uiSize = (oResult.len() as f64/iLineLength as f64).ceil() as usize;
                            for iLineIndex in 0..uiSize {
                                fnLeftByteShift(oResult, iLineIndex, iLineLength, oKeyByteArray[iKeyIndex] as usize+iMethodIndex as usize);
                            }
                        }
                        1 => {
                            let uiSize = (oResult.len() as f64/iLineLength as f64).ceil() as usize;
                            for iLineIndex in 0..uiSize {
                                fnRightByteShift(oResult, iLineIndex, iLineLength, oKeyByteArray[iKeyIndex] as usize+iMethodIndex as usize);
                            }
                        }
                        2 => {
                            for iIndex in 0..oResult.len() {
                                cByte = oResult[iIndex];
                                fnLeftBitShift(&mut cByte, oKeyByteArray[iKeyIndex] as usize+iMethodIndex as usize);
                                oResult[iIndex] = cByte;
                            }
                        }
                        3 => {
                            for iIndex in 0..oResult.len() {
                                cByte = oResult[iIndex];
                                fnRightBitShift(&mut cByte, oKeyByteArray[iKeyIndex] as usize+iMethodIndex as usize);
                                oResult[iIndex] = cByte;
                            }
                        }
                        4 => {
                            for iIndex in 0..oResult.len() {
                                oResult[iIndex] = oResult[iIndex] ^ oKeyByteArray[(iIndex+iMethodIndex) % oKeyByteArray.len()];
                            }
                        }
                        _ => {

                        }
                    }
                }
            }
        }
    }

    return RESULT_SUCCESS;
}

pub fn fnDecrypt() {

}

fn fnLeftByteShift(oData: &mut Vec<u8>, iLineNumber: usize, iLineLength: usize, iShift: usize) {
    let iLinesCount: usize = oData.len() / iLineLength;
    let iAllLinesCount: usize = (oData.len() as f64/ iLineLength as f64).ceil() as usize;

    let iLineNumberRounded = iLineNumber % iAllLinesCount;
    let mut iShiftRounded = iShift;

    let iPosition: usize = iLineNumberRounded*iLineLength;
    let iNextLinePosition: usize = (iLineNumberRounded+1)*iLineLength;

    println!("{:?}", "--------------------------------");
    println!("oData.len() = {:?}", oData.len());
    println!("iLinesCount = {:?}", iLinesCount);
    println!("iAllLinesCount = {:?}", iAllLinesCount);
    println!("iLineNumberRounded = {:?}", iLineNumberRounded);
    println!("iShiftRounded = {:?}", iShiftRounded);
    println!("iPosition = {:?}", iPosition);
    println!("iNextLinePosition = {:?}", iNextLinePosition);

    if iNextLinePosition>oData.len() {
        // let mut iCurrentLineLength = oData.len() % iPosition + 1;
        let iCurrentLineLength = oData.len() % iLineLength;
        iShiftRounded = iShiftRounded % iCurrentLineLength;

        println!("{:?}", "1 --------------------------------");
        println!("iCurrentLineLength = {:?}", iCurrentLineLength);
        println!("iShiftRounded = {:?}", iShiftRounded);

        if iShiftRounded==0 {
            return;
        }

        if iCurrentLineLength==1 {
            return;
        }

        for iIndex in 0..iShiftRounded {
            let iTemp: u8 = oData[iPosition];

            println!("{:?}", "1.1 --------------------------------");
            println!("iPosition+1 = {:?}", iPosition+1);
            println!("iPosition+iCurrentLineLength = {:?}", iPosition+iCurrentLineLength);

            for iShiftIndex in iPosition+1..iPosition+iCurrentLineLength {
                oData[iShiftIndex-1] = oData[iShiftIndex];
            }

            println!("iPosition+iCurrentLineLength-1 = {:?}", iPosition+iCurrentLineLength-1);

            oData[iPosition+iCurrentLineLength-1] = iTemp;
        }
    } else {
        iShiftRounded = iShiftRounded % iLineLength;

        println!("{:?}", "2 --------------------------------");
        println!("iShiftRounded = {:?}", iShiftRounded);

        if iShiftRounded==0 {
            return;
        }

        for iIndex in 0..iShiftRounded {
            let iTemp: u8 = oData[iPosition];

            println!("{:?}", "2.1 --------------------------------");
            println!("iPosition+1 = {:?}", iPosition+1);
            println!("iNextLinePosition = {:?}", iNextLinePosition);

            for iShiftIndex in iPosition+1..iNextLinePosition {
                oData[iShiftIndex-1] = oData[iShiftIndex];
            }

            println!("iNextLinePosition-1 = {:?}", iNextLinePosition-1);

            oData[iNextLinePosition-1] = iTemp;
        }
    }
}

fn fnRightByteShift(oData: &mut Vec<u8>, iLineNumber: usize, iLineLength: usize, iShift: usize) {
    let iLinesCount: usize = oData.len() / iLineLength;
    let iAllLinesCount: usize = (oData.len() as f64/ iLineLength as f64).ceil() as usize;

    let iLineNumberRounded = iLineNumber % iAllLinesCount;
    let mut iShiftRounded = iShift;

    let iPosition: usize = iLineNumberRounded*iLineLength;
    let iNextLinePosition: usize = (iLineNumberRounded+1)*iLineLength;

    if iNextLinePosition>oData.len() {
        // let mut iCurrentLineLength = oData.len() % iPosition + 1;
        let iCurrentLineLength = oData.len() % iLineLength;
        iShiftRounded = iShiftRounded % iCurrentLineLength;

        if iShiftRounded==0 {
            return;
        }

        if iCurrentLineLength==1 {
            return;
        }

        for iIndex in 0..iShiftRounded {
            let iTemp: u8 = oData[iPosition+iCurrentLineLength-1];
            for iShiftIndex in iPosition+iCurrentLineLength-2..iPosition {
                oData[iShiftIndex+1] = oData[iShiftIndex];
            }
            oData[iPosition] = iTemp;
        }
    } else {
        iShiftRounded = iShiftRounded % iLineLength;

        if iShiftRounded==0 {
            return;
        }

        for iIndex in 0..iShiftRounded {
            let iTemp: u8 = oData[iNextLinePosition-1];
            for iShiftIndex in iNextLinePosition-2..iPosition {
                oData[iShiftIndex+1] = oData[iShiftIndex];
            }
            oData[iPosition] = iTemp;
        }
    }
}

fn fnLeftBitShift(ucByte: &mut u8, iShift: usize) {
    let iShiftRounded = iShift % 7 + 1;

    if iShiftRounded==0 {
        return;
    }

    *ucByte = (*ucByte << iShiftRounded) | (*ucByte >> (8 - iShiftRounded));
}

fn fnRightBitShift(ucByte: &mut u8, iShift: usize) {
    let iShiftRounded = iShift % 7 + 1;

    if iShiftRounded==0 {
        return;
    }

    *ucByte = (*ucByte >> iShiftRounded) | (*ucByte << (8 - iShiftRounded));
}