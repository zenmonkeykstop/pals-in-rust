
// MT19937 constants as per Wikipedia entry
const W: u32 = 32; 
const N: usize = 624; 
const M: u32 = 397; 
const R: u32 = 31; 
const A: u64 = 0x9908B0DF;
const U: u64 = 11;
const D: u64 = 0xFFFFFFFF;
const S: u64 = 7;
const B: u64 = 0x9D2C5680;
const T: u64 = 15;
const C: u64 = 0xEFC60000;
const L: u64 = 18;
const F: u64 = 1812433253;

fn u64_lowest_n_bits (b: u64, n: usize) -> u64 {
    if n >= 64 {
        return b;
    }
    return b & ((1 << n) -1);
}

pub struct Twist19937 {
    mt: [u64; N],
    index: u64,
    lower_mask: u64,
    upper_mask: u64,
}

impl Twist19937 {

    pub fn new() -> Twist19937 {
        Twist19937 {
            mt: [0; N],
            index: N as u64 + 1,
            lower_mask: 0,
            upper_mask: 0,
        }
    }

    pub fn seed (&mut self, seed:u64) {
        self.index = N as u64;
        self.mt[0] = seed;

        self.lower_mask = (1<<R) -1;
        self.upper_mask = u64_lowest_n_bits(!self.lower_mask, W as usize);
        for i in 1..N {
            self.mt[i] = u64_lowest_n_bits(F*(self.mt[i-1]^(self.mt[i-1] >> (W-2))) + (i as u64), W as usize);
        }
    }
    
    pub fn getnum(&mut self) -> u32 {
        if self.index >= N as u64 {
            if self.index > N as u64 {
                panic!("Generator was never seeded!");
            }
            self.twist();
        }

        let mut y = self.mt[self.index as usize];
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);
        self.index += 1;
        return u64_lowest_n_bits(y, W as usize) as u32;
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x: u64 = (self.mt[i] & self.upper_mask) + (self.mt[(i+1) %N] & self.lower_mask);
            let mut x_a = x >>1;
            if (x%2) !=0 {
                x_a = x_a^A;
            }
            self.mt[i] = self.mt[(i+(M as usize)) % N] ^ x_a;
        }
        self.index = 0;
    }
}

// Unit tests go in the same file in Rust ... craaazy
#[test]
fn test_lower_n_bits() {
    let t: u64 = u64_lowest_n_bits(49, 3);
    assert_eq!(t, 1);
}


#[test]
fn test_mersenne_value() {
    let mut boo: Twist19937 = Twist19937::new();
    boo.seed(5489);
    let mut u: u32 =0; 
    for _ in 0..10000 {
        u = boo.getnum()
    }
    assert_eq!(u,4123659995);
}
