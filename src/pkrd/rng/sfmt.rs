#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sfmt {
    index: usize,
    sfmt: [u32; 624],
}

impl Sfmt {
    pub fn new(seed: u32) -> Self {
        let mut rng = Self {
            sfmt: [0; 624],
            index: 624,
        };
        rng.init(seed);

        rng
    }

    fn init(&mut self, seed: u32) {
        let mut inner = seed & 1;
        self.sfmt[0] = seed;

        let mut seed = seed;
        for i in 1..624 {
            seed = (seed ^ (seed >> 30))
                .wrapping_mul(0x6c078965)
                .wrapping_add(i);
            self.sfmt[i as usize] = seed;
        }

        inner ^= self.sfmt[3] & 0x13c9e684;
        inner ^= inner >> 16;
        inner ^= inner >> 8;
        inner ^= inner >> 4;
        inner ^= inner >> 2;
        inner ^= inner >> 1;
        self.sfmt[0] ^= !inner & 1;
    }

    pub(crate) fn get_current_state(&self) -> u64 {
        let index = if self.index != 624 { self.index } else { 0 };
        let low = self.sfmt[index] as u64;
        let high = self.sfmt[index + 1] as u64;

        low | (high << 32)
    }

    pub fn next(&mut self) -> u64 {
        // Get state before shuffle
        // Needed for gen 7 compatbility
        let state = self.get_current_state();

        if self.index == 624 {
            self.shuffle();
        }

        self.index += 2;

        state
    }

    fn shuffle(&mut self) {
        let mut b = 488;
        let mut c = 616;
        let mut d = 620;

        for a in (0..624).step_by(4) {
            self.sfmt[a + 3] ^= (self.sfmt[a + 3] << 8)
                ^ (self.sfmt[a + 2] >> 24)
                ^ (self.sfmt[c + 3] >> 8)
                ^ ((self.sfmt[b + 3] >> 11) & 0xbffffff6)
                ^ (self.sfmt[d + 3] << 18);
            self.sfmt[a + 2] ^= (self.sfmt[a + 2] << 8)
                ^ (self.sfmt[a + 1] >> 24)
                ^ (self.sfmt[c + 3] << 24)
                ^ (self.sfmt[c + 2] >> 8)
                ^ ((self.sfmt[b + 2] >> 11) & 0xbffaffff)
                ^ (self.sfmt[d + 2] << 18);
            self.sfmt[a + 1] ^= (self.sfmt[a + 1] << 8)
                ^ (self.sfmt[a] >> 24)
                ^ (self.sfmt[c + 2] << 24)
                ^ (self.sfmt[c + 1] >> 8)
                ^ ((self.sfmt[b + 1] >> 11) & 0xddfecb7f)
                ^ (self.sfmt[d + 1] << 18);
            self.sfmt[a] ^= (self.sfmt[a] << 8)
                ^ (self.sfmt[c + 1] << 24)
                ^ (self.sfmt[c] >> 8)
                ^ ((self.sfmt[b] >> 11) & 0xdfffffef)
                ^ (self.sfmt[d] << 18);

            c = d;
            d = a;
            b += 4;
            if b == 624 {
                b = 0;
            }
        }

        self.index = 0;
    }
}

impl Default for Sfmt {
    fn default() -> Self {
        Self {
            index: 0,
            sfmt: [0; 624],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shuffle() {
        let mut rng = Sfmt::new(0x7725e5e1);
        for _ in 0..1000 {
            rng.next();
        }

        let result = rng.next();
        assert_eq!(result, 0xd7efa47e23000ac8);
    }

    #[test]
    fn test_next_should_return_state_before_shuffle() {
        let mut rng = Sfmt::new(0xc91cc389);
        for _ in 0..624 {
            rng.next();
        }

        let result = rng.next();
        assert_eq!(result, 0xb5618d99ce90d534);
    }
}
