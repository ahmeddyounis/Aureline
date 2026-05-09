use std::ops::Range;

#[derive(Debug, Clone)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        let seed = if seed == 0 {
            0xBAD5_EED5_EED5_EED5
        } else {
            seed
        };
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn next_usize(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }
        (self.next_u64() % (upper as u64)) as usize
    }
}

pub fn random_range(rng: &mut XorShift64, len: usize) -> Range<usize> {
    if len == 0 {
        return 0..0;
    }
    let a = rng.next_usize(len.saturating_add(1));
    let b = rng.next_usize(len.saturating_add(1));
    let (start, end) = if a <= b { (a, b) } else { (b, a) };
    start..end
}
