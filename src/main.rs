extern crate hex;
extern crate base64;
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


    //ex1.6
    let file2 = match File::open("6.txt") {
       Ok(file2) => file2,
       Err(e) => panic!("Error opening file: {}", e),
    };

    let mut v1_6: Vec<u8> = Vec::new();
    for line in BufReader::new(file2).lines() {
        v1_6.append(&mut base64::decode(&line.unwrap()).unwrap());
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
            let a: Vec<u8> = v1_6[i*len..(i*len)+len].to_vec();
            let b: Vec<u8> = v1_6[(i+1)*len..((i+1)*len)+len].to_vec();
            avg_ham += pals::hamming_dist(a, b) as f64;
            c += 1;
        }
        hams.push((len, avg_ham/(c*len) as f64));
    }
    hams.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("Top 5 candidate keylengths: \n{:?}", hams[0..5].to_vec());
    let testlen = hams[0].0;

    println!("Candidate length is {}", testlen);

    let mut testkey: Vec<u8> = Vec::new();
    for mut i in 0..testlen {
        // run the single_char_xor test against the slice
        // append the winner to the testkey
        testkey.push(pals::decrypt_single_xor(&pals::pick_nth_from_vec(v1_6.clone(), testlen as i32, i as i32), 0)[0].key as u8);
    }
    println!("Key might be: \"{}\"", String::from_utf8_lossy(&testkey));
    println!("---\nText might be:\n{}", String::from_utf8_lossy(&pals::xor_vectors(&v1_6, &testkey)));

}
