extern crate base64;
use std::u8;

pub fn hex_to_bytes(hex: String) -> Vec<u8> {
    if hex.len()%2 == 1 || hex.len() == 0 {
        panic!("hex string can't be broken into bytes: {}", hex);
    }

    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => panic!("Input error: {}", e),
        };
    }
    return bytes;
}

pub fn hex_to_base64(hex: String) -> String {

    let bytes = hex_to_bytes (hex);
    self::base64::encode(&bytes)
}


pub fn xor_vectors(plaintext: &Vec<u8>, key: &Vec<u8>) -> Vec<u8>  {

    let key_cycle = key.iter().cycle();
    plaintext.iter().zip(key_cycle).map(|(&a, b)| a^b).collect::<Vec<u8>>()
}


fn hamming_byte(b1: u8, b2: u8) -> i32 {

    let mut t = b1^b2;
    
    // Kernighan's bit-counting algorithm
    let mut c: i32 = 0;
    while t != 0 {
        t &= t-1;
        c += 1;
    }
    return c;
}


pub fn hamming_dist(s1: Vec<u8>, s2: Vec<u8> ) -> i32 {

    if s1.len() != s2.len() {
        panic!("Can't calculate Hamming dist - vectors not equal!");
    }
    return s1.iter().zip(s2.iter()).map(|(a, b)| hamming_byte(*a, *b)).sum::<i32>();

}

pub fn pick_nth_from_vec<T>(v: Vec<T>, n: i32, offset: i32) -> Vec<T> {
    return v.into_iter().enumerate().filter(|&(i, _)| i as i32 % n == offset).map(|(_,v)| v).collect();
}

// TESTS START
// Unit tests go in the same file in Rust ... craaazy
#[test]
fn test_hex_to_bytes() {

    let test_str: String = "ffdd".to_string();
    assert_eq!(hex_to_bytes(test_str).len(), 2);
}

#[test]
fn test_hamming_distance_val() {

    let t1: Vec<u8> = String::from("this is a test").into_bytes();
    let t2: Vec<u8> = String::from("wokka wokka!!!").into_bytes();
    assert_eq!(hamming_dist(t1, t2), 37)
}

#[test]
fn test_vector_pick_nth() {
    let v: Vec<i32> = vec![1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5];
    assert_eq!(pick_nth_from_vec(v.clone(), 5, 4), vec![5,5,5,5]);
    assert_eq!(pick_nth_from_vec(v.clone(), 5, 0), vec![1,1,1,1]);
    println!("{:?}", v);
    let c = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string(), "durian".to_string(), "etrog".to_string(), "fig".to_string(), "grape".to_string(), "honeydew".to_string()];
    assert_eq!(pick_nth_from_vec(c.clone(), 2, 0), vec!["apple".to_string(), "cherry".to_string(), "etrog".to_string(),"grape".to_string()]);
}

