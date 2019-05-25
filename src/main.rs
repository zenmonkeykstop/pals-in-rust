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
    let ex1_3_ct = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let bob: Vec<pals::SingleXORTest> = pals::decrypt_single_xor(&pals::hex_to_bytes(ex1_3_ct), 0);
    println!("1.3: {:?}", bob);

    // ex1.4
    // load files, run single_char_detect against all of them, pick the top n, boom.

    let file = match File::open("4.txt") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut e: Vec<pals::SingleXORTest> = Vec::new();
    for (num, line) in BufReader::new(file).lines().enumerate() {
        let l = pals::hex_to_bytes(String::from(line.unwrap()));
        
        e.append(&mut pals::decrypt_single_xor(&l, num as i32));

    }
    e.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    e.truncate(1);
    for elem in e {
        println!("1.4: {:?}", elem);
    }

    // ex1.5
    let ex1_5_pt: Vec<u8> = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal").into_bytes();
    let ex1_5_key: Vec<u8> = String::from("ICE").into_bytes();
 
    println!("1.5: {}", hex::encode(pals::xor_vectors(&ex1_5_pt, &ex1_5_key)));

    // ex1.6
   
    // what's our target length for the key?
    // for lengths from 2 to 40
    //   grab the first 3 key lengths from vector, compute hamming dist for l1-l2, l2-l3, average
    //   them
    // pick the lowest score
    //
    // for the chosen key length:
    //   slice out every keylen-th char, offset by index of loop
    //   get the best single_xor_test
    // stick em all together, try xor_vectors with it, display a couple of lines...

}
