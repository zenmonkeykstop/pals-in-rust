extern crate aes;
extern crate xor;
extern crate utils;

use std::io::{ BufRead, BufReader };
use std::fs::{File};

pub fn ex9() {
    // ex2.9
        let t: Vec<u8> = String::from("YELLOW SUBMARINE").into_bytes();
        let r = aes::pad_pkcs7(&t, 20);

    let t1 = String::from_utf8(t).expect("Found invalid UTF-8");
    println!("string: {}", t1);
    println!("string length: {}", t1.len());
    let r1 = String::from_utf8(r).expect("Found invalid UTF-8");
    println!("result: {}", r1);
    println!("result length: {}", r1.len());
}

pub fn ex10() {
    let k = b"YELLOW SUBMARINE";
    let iv = vec![0 as u8; aes::BLOCKSIZE];

    let f = match File::open("files/10.txt") {
       Ok(f) => f,
       Err(e) => panic!("Error opening file: {}", e),
    };

    let mut ct: Vec<u8> = Vec::new();
    for line in BufReader::new(f).lines() {
        ct.append(&mut base64::decode(&line.unwrap()).unwrap());
    }
    
    let pt = aes::cbc_decrypt(&ct, &iv, k);
    
    print!("result:\n{}", std::str::from_utf8(&pt).unwrap());
}

pub fn ex11() {
    print!("ayyyooo challenge 11");
}
