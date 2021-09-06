pub struct SFMT {
    index: usize,
    sfmt: [u32; 624],
}

impl SFMT {
    pub fn new(seed: u32) -> SFMT {
        let mut rng = SFMT {
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
            seed = 0x6c078965u32
                .wrapping_mul(seed ^ (seed >> 30))
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

    pub fn next(&mut self) -> u64 {
        if self.index == 624 {
            self.shuffle();
        }

        let low = self.sfmt[self.index] as u64;
        self.index += 1;
        let high = self.sfmt[self.index] as u64;
        self.index += 1;

        low | (high << 32)
    }

    fn shuffle(&mut self) {
        let mut b = 488;
        let mut c = 616;
        let mut d = 620;

        for a in (0..624).step_by(4) {
            self.sfmt[a + 3] ^= (self.sfmt[a + 3] << 8) ^ (self.sfmt[a + 2] >> 24) ^ (self.sfmt[c + 3] >> 8) ^ ((self.sfmt[b + 3] >> 11) & 0xbffffff6) ^ (self.sfmt[d + 3] << 18);
            self.sfmt[a + 2] ^= (self.sfmt[a + 2] << 8) ^ (self.sfmt[a + 1] >> 24) ^ (self.sfmt[c + 3] << 24) ^ (self.sfmt[c + 2] >> 8) ^ ((self.sfmt[b + 2] >> 11) & 0xbffaffff) ^ (self.sfmt[d + 2] << 18);
            self.sfmt[a + 1] ^= (self.sfmt[a + 1] << 8) ^ (self.sfmt[a] >> 24) ^ (self.sfmt[c + 2] << 24) ^ (self.sfmt[c + 1] >> 8) ^ ((self.sfmt[b + 1] >> 11) & 0xddfecb7f) ^ (self.sfmt[d + 1] << 18);
            self.sfmt[a] ^= (self.sfmt[a] << 8) ^ (self.sfmt[c + 1] << 24) ^ (self.sfmt[c] >> 8) ^ ((self.sfmt[b] >> 11) & 0xdfffffef) ^ (self.sfmt[d] << 18);

            c = d;
            d = a;
            b += 4;
            if b == 624
            {
                b = 0;
            }
        }

        self.index = 0;
    }
}
