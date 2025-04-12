use crate::rng::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TinyMT {
    initial_state: [u32; 4],
    state: [u32; 4],
}

impl TinyMT {
    pub fn new(seed: u32) -> Self {
        let mut state = [seed, 0x8f7011ee, 0xfc78ff1f, 0x3793fdff];

        for i in 1..8 {
            state[i & 3] ^= (state[(i - 1) & 3] ^ (state[(i - 1) & 3] >> 30))
                .wrapping_mul(1812433253)
                .wrapping_add(i as u32);
        }

        let mut rng = Self {
            state,
            initial_state: state,
        };

        for _ in 0..8 {
            rng.next_state();
        }

        rng
    }

    fn next_state(&mut self) {
        let mut y = self.state[3];
        let mut x = (self.state[0] & 0x7FFFFFFF) ^ self.state[1] ^ self.state[2];

        x ^= x << 1;
        y ^= (y >> 1) ^ x;

        self.state[0] = self.state[1];
        self.state[1] = self.state[2] ^ ((y & 1) * 0x8f7011ee);
        self.state[2] = x ^ (y << 10) ^ ((y & 1) * 0xfc78ff1f);
        self.state[3] = y;
    }

    fn get_state(&self) -> [u32; 4] {
        self.state
    }

    pub fn initial_state(&self) -> &[u32; 4] {
        &self.initial_state
    }
}

impl Rng for TinyMT {
    type Seed = u32;
    type CurrentState = [u32; 4];

    fn new(seed: Self::Seed) -> Self {
        TinyMT::new(seed)
    }

    fn next_state(&mut self) -> Self::CurrentState {
        self.next_state();
        self.get_state()
    }

    fn current_state(&mut self) -> Self::CurrentState {
        self.get_state()
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
            Rng::next_state(&mut rng),
            [0x5a385202, 0xd9905227, 0x90ffb4e5, 0x3dc72b8f]
        );
    }

    #[test]
    fn should_not_fail_from_state_overflow() {
        let mut rng = TinyMT::new([0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff]);
        assert_eq!(
            Rng::next_state(&mut rng),
            [0xffffffff, 0x708fee11, 0x7c78fb1e, 0x00000001]
        );
    }
}
