extern crate aes;
extern crate xor;
extern crate utils;

use std::io::{ BufRead, BufReader };
use std::fs::{File};
use rand::prelude::*;

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

fn rnd_bytes (l: u8) -> Vec<u8> {
    let mut random_bytes = Vec::new();
    for _i in 0..l {
        random_bytes.push(rand::thread_rng().gen::<u8>());
    }
    return random_bytes;
}


fn ecb_oracle(p: &[u8]) -> bool {
    let mut pt = p.to_vec();
    //decide whether we're doing ecb or cbc
    let is_ecb = rand::thread_rng().gen_bool(0.5);
    println!("You don't know this but this is ECB: {}", is_ecb);

    
    // generate a key
    let key = rnd_bytes(16);
    println!("key is: {}", hex::encode(&key));
    println!("key length is {}", key.len());
    // append random bytes before/after plaintext
    let prequel = rnd_bytes(rand::thread_rng().gen_range(5,10));
    let epilog = rnd_bytes(rand::thread_rng().gen_range(5,10));
    println!("{}, {}", hex::encode(&prequel), hex::encode(&epilog));
    // run the appropriate encryption
    let mut ct = Vec::new();
    if is_ecb {
        ct = aes::ecb_encrypt(&pt, &key);
    } else {
        let iv = rnd_bytes(16);
        ct = aes::cbc_encrypt(&pt, &iv, &key)
    }

    // look for repeating blocks, (this will work with
    // a chosen plaintext of repeating elements at least 3
    // blocks long:
    // |5-10byte+..AAAA|AA..AA|AA..AA|AAAA..5-10byte|
    // If detected
    return true;
    //else return false;
}

//side note: this exercise's wording is frustrating to me - are
// you allowed to change the plaintext or not? It seems like you
// could only make a statistical guess if not. So I'm assuming yes.
pub fn ex11() {
    ecb_oracle(b"banana");
}
