#![cfg_attr(not(test), no_std)]

// Simon showed me this webpage to help: https://en.wikipedia.org/wiki/Xorshift

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct Random {
    state: usize
}

impl Random {
    pub fn new(seed: usize) -> Self {
        Random {
            state: seed
        }
    }

    pub fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
	    x ^= x >> 7;
	    x ^= x << 17;
        self.state = x;
        x
    }
}