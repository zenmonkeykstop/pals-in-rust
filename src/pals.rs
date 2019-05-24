extern crate base64;
use std::u8;
use std::cmp;

//ASSUMING THAT CRYPTOPAL KEYS ARE PRINTABLE ASCII - only time will tell!
const KEYSPACE: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

const ALPHALENGTH: usize = 27;
const CHARFREQ: [f64; ALPHALENGTH] =  [ 
    0.0651738, 0.0124248, 0.0217339, 0.0349835, 
    0.1041442, 0.0197881, 0.0158610, 0.0492888, 
    0.0558094, 0.0009033, 0.0050529, 0.0331490,
    0.0202124, 0.0564513, 0.0596302, 0.0137645, 
    0.0008606, 0.0497563, 0.0515760, 0.0729357, 
    0.0225134, 0.0082903, 0.0171272, 0.0013692, 
    0.0145984, 0.0007836, 0.1918182]; 

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

pub fn get_chi(s: &String) -> f64 {
    let mut cc: [f64; ALPHALENGTH]  = [0.0; ALPHALENGTH];
    let mut tc: i32 = 0;
    let mut score = 0.0;

    for c in s.chars() {
            if c.is_ascii_alphabetic()  {
                cc[(c.to_ascii_uppercase() as usize -'A' as usize)] += 1.0;
                tc +=1;
            } 
            else if c.is_ascii_whitespace() {
                cc[ALPHALENGTH-1] += 1.0;
                tc +=1;
            } else {
                score += 1.0; // penalizing non-alphabetic HEAVILY - this is no longer a proper chi-square!
            }
    }
    tc = cmp::max(1, tc);


    for i in 0..ALPHALENGTH {
        cc[i] = cc[i]/(tc as f64);
        score += ((cc[i] - CHARFREQ[i])*(cc[i] - CHARFREQ[i]))/CHARFREQ[i];
    }

    // println!("{}", s);
    // println!("{:?}, {}", cc, tc);
    
    return score; 
}


#[derive(Debug)]
pub struct SingleXORTest {
    pub score: f64,
    pub key: char,
    pub string: String,
    pub lnum: i32,
}

pub fn decrypt_single_xor(s: &Vec<u8>, line_num: i32 ) -> Vec<SingleXORTest> {
    const MAX_RESULTS: usize = 3;
    let mut v: Vec<SingleXORTest> = Vec::new();
    for key in KEYSPACE.chars() {
 
       let t = s.iter().map(|&a | a^key as u8 ).collect::<Vec<u8>>();
       let ts = String::from_utf8_lossy(&t).to_string();
       v.push(SingleXORTest { score: get_chi(&ts), key: key, string: ts.to_string().to_owned(), lnum: line_num});
    }
    v.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
     
    if v.len() > MAX_RESULTS {
        v.truncate(MAX_RESULTS);
        return v;
    } else {
        return v;
    }
}

pub fn hamming_byte(&b1: &u8, &b2: &u8) -> i32 {
    let mut t = b1 ^b2;
    
    // Kernighan's bit-counting algorithm
    let mut c: i32 = 0;
    while t != 0 {
        t &= t-1;
        c += 1;
    }
    return c;
}

pub fn hamming_dist(s1: &Vec<u8>, s2: &Vec<u8> ) -> i32 {

    if s1.len() != s2.len() {
        panic!("Can't calculate Hamming dist - vectors not equal!");
    }
    return s1.iter().zip(s2.iter()).map(|(&a, b)| hamming_byte(&a, b)).sum::<i32>();
}


// TESTS START
// Unit tests go in the same file in Rust ... craaazy
#[cfg(test)]
mod tests {
    use pals;
    #[test]
    fn test_hex_to_bytes() {
        let test_str: String = "ffdd".to_string();
        assert_eq!(pals::hex_to_bytes(test_str).len(), 2);
    }

    #[test]
    fn test_hamming_distance_val() {
        let t1: Vec<u8> = String::from("this is a test").into_bytes();
        let t2: Vec<u8> = String::from("wokka wokka!!!").into_bytes();
        assert_eq!(pals::hamming_dist(&t1, &t2), 37)
    }
}

