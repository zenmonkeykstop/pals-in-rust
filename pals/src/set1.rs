extern crate twist;
extern crate aes;
extern crate xor;
extern crate utils;

extern crate hex;
extern crate base64;
extern crate openssl;

use std::io::{ BufRead, BufReader };
use std::fs::{File};
// use set1::openssl::symm::{decrypt, Cipher};


pub fn ex1() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("1.1: {}", utils::hex_to_base64(s.to_string()));
}

pub fn ex2() {
    let pt = utils::hex_to_bytes("1c0111001f010100061a024b53535009181c".to_string());
    let k = utils::hex_to_bytes("686974207468652062756c6c277320657965".to_string());
    println!("1.2: {}", hex::encode(utils::xor_vectors(&pt, &k)));
}

pub fn ex3() {
    let ct = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let pt: Vec<xor::SingleXORTest> = xor::decrypt_single_xor(&utils::hex_to_bytes(ct), 0);
    println!("1.3: {:?}", pt);
}

pub fn ex4() {    // ex1.4
    // load files, run single_char_detect against all of them, pick the top n, boom.

    let f = match File::open("files/4.txt") {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut e: Vec<xor::SingleXORTest> = Vec::new();
    for (num, line) in BufReader::new(f).lines().enumerate() {
        let l = utils::hex_to_bytes(String::from(line.unwrap()));
        
        e.append(&mut xor::decrypt_single_xor(&l, num as i32));

    }
    e.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    e.truncate(1);
    for elem in e {
        println!("1.4: {:?}", elem);
    }
}

pub fn ex5() {
    let pt: Vec<u8> = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal").into_bytes();
    let k: Vec<u8> = String::from("ICE").into_bytes();
 
    println!("1.5: {}", hex::encode(utils::xor_vectors(&pt, &k)));
}

pub fn ex6() {
    let f = match File::open("files/6.txt") {
       Ok(f) => f,
       Err(e) => panic!("Error opening file: {}", e),
    };

    let mut ct: Vec<u8> = Vec::new();
    for line in BufReader::new(f).lines() {
        ct.append(&mut base64::decode(&line.unwrap()).unwrap());
    }

    
    const MAX_KEY_LEN: usize = 40;
    const SUM_OVER_HAMS: usize = 10; // how many pairs of hamming dists to compare
                                     // note that ex1.6 instructions suggest like 2, which did not
                                     // work for me!
                                     
    let mut hams: Vec<(usize, f64)> = Vec::new();
    for len in 2..MAX_KEY_LEN {
        let mut avg_ham: f64 = 0.0;
        let mut c: usize = 0;
        for i in 0..SUM_OVER_HAMS {
            let a: Vec<u8> = ct[i*len..(i*len)+len].to_vec();
            let b: Vec<u8> = ct[(i+1)*len..((i+1)*len)+len].to_vec();
            avg_ham += utils::hamming_dist(a, b) as f64;
            c += 1;
        }
        hams.push((len, avg_ham/(c*len) as f64));
    }
    hams.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("Top 5 candidate keylengths: \n{:?}", hams[0..5].to_vec());
    let testlen = hams[0].0;

    println!("Candidate length is {}", testlen);

    let mut testkey: Vec<u8> = Vec::new();
    for i in 0..testlen {
        // run the single_char_xor test against the slice
        // append the winner to the testkey
        testkey.push(xor::decrypt_single_xor(&utils::pick_nth_from_vec(ct.clone(), testlen as i32, i as i32), 0)[0].key as u8);
    }
    println!("Key might be: \"{}\"", String::from_utf8_lossy(&testkey));
    println!("---\nText might be:\n{}", String::from_utf8_lossy(&utils::xor_vectors(&ct, &testkey)));
}

/*
pub fn ex7a() {
    let k = "YELLOW SUBMARINE".as_bytes();

    let f = match File::open("files/7.txt") {
       Ok(f) => f,
       Err(e) => panic!("Error opening file: {}", e),
    };

    let mut ct: Vec<u8> = Vec::new();
    for line in BufReader::new(f).lines() {
        ct.append(&mut base64::decode(&line.unwrap()).unwrap());
    }

    let cipher = Cipher::aes_128_ecb();
    let pt = decrypt(cipher, k, Some(k), &ct).unwrap();

    println!("{}", String::from_utf8(pt).unwrap());
}
*/

pub fn ex7() {
    let k = b"YELLOW SUBMARINE";

    let f = match File::open("files/7.txt") {
       Ok(f) => f,
       Err(e) => panic!("Error opening file: {}", e),
    };

    let mut ct: Vec<u8> = Vec::new();
    for line in BufReader::new(f).lines() {
        ct.append(&mut base64::decode(&line.unwrap()).unwrap());
    }
    let chunks: Vec<&[u8]> = ct.chunks(16).collect();

    let mut pt: Vec<u8> = Vec::new();

    
    for chunk in &chunks {
         let mut ptc =  aes::aes_decrypt_block(&chunk, k);
         pt.append(&mut ptc);
    }
   
    println!("{}", String::from_utf8(pt).unwrap());
}
    

pub fn ex8() {
    let f = match File::open("files/8.txt") {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };
    for (number, line) in BufReader::new(f).lines().enumerate() {
        let testbytes = line.unwrap().into_bytes();
        let mut chunks : Vec<&[u8]> = testbytes.chunks(16).collect();
        chunks.sort_unstable();
        let orig_len = chunks.len();
        chunks.dedup();
        let count = orig_len - chunks.len();
        if count > 0 {
            println!("line {}: {} matching chunks", number, count);
        }
    }
}
