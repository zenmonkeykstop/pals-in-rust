use std::u8;

pub fn pad_pkcs7(b: &Vec<u8>,  l: usize) -> Vec<u8> {
    let mut vec = b.clone();
    if vec.len() > l {
        panic!("input block size is greater than padded size");
    } else if vec.len() == l {
        let mut padding = vec![l as u8; l];
        vec.append(&mut padding);
    } else if vec.len() < l {
        let mut padding = vec![(l-vec.len()) as u8; l-vec.len()];
        vec.append(&mut padding);
    }
    return vec;
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
