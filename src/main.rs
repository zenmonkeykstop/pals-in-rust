extern crate hex;
use std::io::{BufRead, BufReader};
use std::fs::File;

mod pals;

fn main() {
    // ex1.1
    let string1_1 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("1.1: {}", pals::hex_to_base64(string1_1.to_string()));

    // ex1.2
    let ex1_2_plaintext = pals::hex_to_bytes("1c0111001f010100061a024b53535009181c".to_string());
    let ex1_2_key       = pals::hex_to_bytes("686974207468652062756c6c277320657965".to_string());
    println!("1.2: {}", hex::encode(pals::xor_vectors(&ex1_2_plaintext, &ex1_2_key)));

    // ex1.3
    let ex1_3_ct = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bob: Vec<pals::SingleXORTest> = pals::decrypt_single_xor(&ex1_3_ct.to_string(), 0);
    println!("1.3: {:?}", bob);

    // ex1.4
    // load files, run single_char_detect against all of them, pick the top 3, boom.

    let file = match File::open("4.txt") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut e: Vec<pals::SingleXORTest> = Vec::new();
    for (num, line) in BufReader::new(file).lines().enumerate() {
        let l = line.unwrap();
        e.append(&mut pals::decrypt_single_xor(&l, num as i32));

    }
    e.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    e.truncate(1);
    for elem in e {
        println!("1.4: {:?}", elem);
    }
}
