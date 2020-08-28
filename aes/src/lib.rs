extern crate utils;
extern crate openssl;

use std::u8;
use openssl::symm::{encrypt, decrypt, Cipher};

pub const BLOCKSIZE: usize = 16;

pub fn pad_pkcs7(b: &Vec<u8>,  l: usize) -> Vec<u8> {
    let mut v = b.clone();
    if v.len() > l {
        panic!("input block size is greater than padded size");
    } else if v.len() == l {
        let mut padding = vec![l as u8; l];
        v.append(&mut padding);
    } else if v.len() < l {
        let mut padding = vec![(l-v.len()) as u8; l-v.len()];
        v.append(&mut padding);
    }
    return v;
}

pub fn unpad_pkcs7(b: &Vec<u8>, l: usize) -> Vec<u8> {
    // todo - need this later, can live without it for now
    let v = b.clone();
    //unpad logic here
    return v;
}

pub fn aes_encrypt_block(b: &[u8], k: &[u8]) -> Vec<u8> {
    if b.len() != BLOCKSIZE {
        panic!("Input length must be {}", BLOCKSIZE);
    }
    let pt = pad_pkcs7(&b.to_vec(), BLOCKSIZE);
    let cipher = Cipher::aes_128_ecb();
    let mut u = encrypt(cipher, k, None, &pt).unwrap();
    u.truncate(BLOCKSIZE);
    return u;
}

pub fn aes_decrypt_block(b: &[u8], k: &[u8]) -> Vec<u8> {
    if b.len() != BLOCKSIZE {
        panic!("Input length must be {}", BLOCKSIZE);
    }

    // append a block of padding, encrypted (openssl decryption needs it)
    let padding = aes_encrypt_block(&[BLOCKSIZE as u8; BLOCKSIZE], k);
    let mut c = b.to_vec();
    c.extend_from_slice(&padding);   

    // do the decryption
    let cipher = Cipher::aes_128_ecb();
    let pt = decrypt(cipher, &k, Some(&k), &c).unwrap();
    //return unpad_pkcs7(&pt, BLOCKSIZE);;
    return pt;
}

pub fn cbc_encrypt(p: &[u8], iv: &[u8], k: &[u8]) -> Vec<u8> {
    // CBC: for first block, xor the plaintext with the IV, then encrypt with key
    // for subsequent blocks, xor the plaintext with the previous cyphertext block, then encrypt
    return p.to_vec();
}

pub fn cbc_decrypt(c: &[u8], iv: &[u8], k: &[u8]) -> Vec<u8> {
    // CBC: for first block, decrypt, then xor with IV to recover the plaintext
    // for subsequent blocks, decrypt, then xor with previous ciphertext block to recover plaintext

    let mut pt: Vec<u8> = Vec::new();

    let mut firstblock: bool = true;

    let mut prevblock: Vec<u8> = Vec::new();

    let blocks: Vec<&[u8]> = c.chunks(BLOCKSIZE).collect();
    for block in &blocks {
        if firstblock { 
            firstblock = false;
            let mut ptc = aes_decrypt_block(&block.to_vec(), &k);
//            let mut ptc = utils::xor_vectors(&iv.to_vec(), &xb);
            prevblock = block.to_vec();
            pt.append(&mut ptc);
        }
        else {
            let xb = aes_decrypt_block(&block.to_vec(), &k);
            let mut ptc = utils::xor_vectors(&xb, &prevblock);
            prevblock = block.to_vec();
            pt.append(&mut ptc);
        }
    }
    return pt.to_vec();
}



// TESTS START
// Unit tests go in the same file in Rust ... craaazy
#[cfg(test)]
mod tests {

#[test]
    fn test_pad_pkcs7() {
        let t1: Vec<u8>  = String::from("YELLOW SUBMARINE").into_bytes();
        assert_eq!(pals::pad_pkcs7(&t1, 20), String::from("YELLOW SUBMARINE\x04\x04\x04\x04").into_bytes());
        assert_eq!(pals::pad_pkcs7(&t1, 16), String::from("YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10").into_bytes());
    }
}
