/*
 *     (w, n, m, r) = (32, 624, 397, 31)
    a = 9908B0DF16
    (u, d) = (11, FFFFFFFF16)
    (s, b) = (7, 9D2C568016)
    (t, c) = (15, EFC6000016)
    l = 18
    */

// MT19937 constants as per Wikipedia
const W: u32 = 32; 
const N: usize = 624; 
const M: u32 = 397; 
const R: u32 = 31; 
const A: u64 = 0x9908B0DF;
const F: u64 = 1812433253;
const U: u64 = 11;
const D: u64 = 0xFFFFFFFF;
const S: u64 = 7;
const B: u64 = 0x9D2C5680;
const T: u64 = 7;
const C: u64 = 0xEFC60000;
const L: u64 = 18;

/*
 *  // Create a length n array to store the state of the generator
 int[0..n-1] MT
 int index := n+1
 const int lower_mask = (1 << r) - 1 // That is, the binary number of r 1's
 const int upper_mask = lowest w bits of (not lower_mask)

 */

pub struct Twist19937 {
    pub mt: [u64; N],
    index: u64,
    pub lower_mask: u64,
    pub upper_mask: u64,
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
        self.upper_mask = ((!self.lower_mask) >> 0) & ((1 << W) -1);
        for i in 1..N {
            self.mt[i] = (F * (self.mt[i-1] ^ (self.mt[i-1] >> (W-2))) + i as u64) & ((1 << W)-1);
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
        return (y & ((1 << W) -1)) as u32;
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

// TESTS START
// Unit tests go in the same file in Rust ... craaazy
#[cfg(test)]
mod tests {

    use twist;
}

