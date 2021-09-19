#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TinyMT {
    state: [u32; 4],
}

impl TinyMT {
    pub fn new(state: [u32; 4]) -> TinyMT {
        TinyMT { state }
    }

    pub fn next_state(&mut self) {
        let mut y = self.state[3];
        let mut x = (self.state[0] & 0x7FFFFFFF) ^ self.state[1] ^ self.state[2];

        x ^= x << 1;
        y ^= (y >> 1) ^ x;

        self.state[0] = self.state[1];
        self.state[1] = self.state[2] ^ ((y & 1) * 0x8f7011ee);
        self.state[2] = x ^ (y << 10) ^ ((y & 1) * 0xfc78ff1f);
        self.state[3] = y;
    }

    pub fn temper(&self) -> u32 {
        let mut t0 = self.state[3];
        let t1 = self.state[0] + (self.state[2] >> 8);

        t0 ^= t1;
        if t1 & 1 == 1 {
            t0 ^= 0x3793fdff;
        }

        t0
    }

    pub fn next(&mut self) -> u32 {
        self.next_state();
        self.temper()
    }

    pub fn get_state(&self) -> [u32; 4] {
        self.state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_random_values() {
        let mut rng = TinyMT::new([0x11112222, 0x33334444, 0x55556666, 0x77778888]);
        for _ in 0..156 {
            rng.next_state();
        }

        assert_eq!(
            rng.get_state(),
            [0x233f3c9d, 0x5a385202, 0x56e043c9, 0x76b46859]
        );
    }
}
