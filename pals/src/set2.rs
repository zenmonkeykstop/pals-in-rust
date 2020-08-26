extern crate twist;
extern crate aes;
extern crate xor;
extern crate utils;

extern crate hex;
extern crate base64;
extern crate openssl;

//use std::io::{ BufRead, BufReader };
// use std::fs::{File};
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

