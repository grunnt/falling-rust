#[derive(Clone, Debug)]
pub struct PseudoRandom {
    pub next: u32,
}

// Quick and dirty pseudo-random number generator
impl PseudoRandom {
    pub fn new() -> Self {
        Self { next: 12345 }
    }

    pub fn next(&mut self) -> u32 {
        self.next ^= self.next << 13;
        self.next ^= self.next >> 17;
        self.next ^= self.next << 5;
        self.next
    }
}
