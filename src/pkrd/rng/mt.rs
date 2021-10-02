#[derive(Clone, Debug, PartialEq)]
pub struct MT {
    index: usize,
    mt: [u32; 624],
}

impl MT {
    pub fn new(seed: u32) -> Self {
        let mut rng = Self::default();
        rng.init(seed);

        rng
    }

    fn blank_mt() -> Self {
        Self {
            mt: [0; 624],
            index: 0,
        }
    }

    fn init(&mut self, seed: u32) {
        self.mt[0] = seed;

        let mut seed = seed;
        for i in 1..624 {
            seed = (seed ^ (seed >> 30))
                .wrapping_mul(0x6c078965)
                .wrapping_add(i);
            self.mt[i as usize] = seed;
        }
    }

    pub fn next(&mut self) -> u32 {
        if self.index == 624 {
            self.shuffle();
        }

        let y = self.mt[self.index];
        self.index += 1;

        // Game doesn't store tempered in memory
        //y ^= y >> 11;
        //y ^= (y << 7) & 0x9d2c5680;
        //y ^= (y << 15) & 0xefc60000;
        //y ^= y >> 18;
        y
    }

    fn shuffle(&mut self) {
        for i in 0..227 {
            let y = (self.mt[i] & 0x80000000) | (self.mt[i + 1] & 0x7fffffff);

            let mut y1 = y >> 1;
            if (y & 1) == 1 {
                y1 ^= 0x9908b0df;
            }

            self.mt[i] = y1 ^ self.mt[i + 397];
        }

        for i in 227..623 {
            let y = (self.mt[i] & 0x80000000) | (self.mt[i + 1] & 0x7fffffff);

            let mut y1 = y >> 1;
            if (y & 1) == 1 {
                y1 ^= 0x9908b0df;
            }

            self.mt[i] = y1 ^ self.mt[i - 227];
        }

        let y = (self.mt[623] & 0x80000000) | (self.mt[0] & 0x7fffffff);

        let mut y1 = y >> 1;
        if (y & 1) == 1 {
            y1 ^= 0x9908B0DF;
        }

        self.mt[623] = y1 ^ self.mt[396];

        self.index = 0;
    }
}

impl Default for MT {
    fn default() -> Self {
        Self::blank_mt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shuffle() {
        let mut rng = MT::new(0xaabbccdd);
        for _ in 0..624 {
            rng.next();
        }

        let result = rng.next();
        assert_eq!(result, 0xd80fcb47);
    }
}
