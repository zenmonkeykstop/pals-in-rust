extern crate openssl;

use std::u8;
use openssl::symm::{encrypt, decrypt, Cipher};

const BLOCKSIZE: usize = 16;

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
