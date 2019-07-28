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
