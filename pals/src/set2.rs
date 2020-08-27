extern crate twist;
extern crate aes;
extern crate xor;
extern crate utils;

extern crate hex;
extern crate base64;
extern crate openssl;

//use std::io::{ BufRead, BufReader };
use std::fs::{File};
// use set1::openssl::symm::{decrypt, Cipher};


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

    // load file, CBC decrypt it, boom
    let ct = match utils::file_to_vec("files/10.txt".to_string()) {
        Ok(v) => v,
        Err(e) => panic!("Couldn't read the ciphertext oh noes: {}", e),
    };

    let  k: Vec<u8> = String::from("YELLOW SUBMARINE").into_bytes();
    let iv = vec![0 as u8; aes::BLOCKSIZE];
    
    let pt = aes::cbc_decrypt(&ct, &k, &iv);
    let s = match std::str::from_utf8(&pt) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    print!("result:\n{}", s);
}
