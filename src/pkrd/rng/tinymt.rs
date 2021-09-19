pub struct TinyMT {
}

impl TinyMT {
    pub fn next_state(state : &mut [u32; 4]) {
        let mut y = state[3];
        let mut x = (state[0] & 0x7FFFFFFF) ^ state[1] ^ state[2];

        x ^= x << 1;
        y ^= (y >> 1) ^ x;

        state[0] = state[1];
        state[1] = state[2] ^ ((y & 1) * 0x8f7011ee);
        state[2] = x ^ (y << 10) ^ ((y & 1) * 0xfc78ff1f);
        state[3] = y;
    }

    pub fn temper(state: &mut [u32; 4]) -> u32 {
        let mut t0 = state[3];
        let t1 = state[0] + (state[2] >> 8);

        t0 ^= t1;
        if t1 & 1 == 1 {
            t0 ^= 0x3793fdff;
        }
        
        t0
    }

    pub fn next(state : &mut [u32; 4]) -> u32 {
        TinyMT::next_state(state);
        TinyMT::temper(state)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_random_values() {
        let mut state = [0x11112222,0x33334444,0x55556666,0x77778888];
        for _ in 0..156 {
            TinyMT::next_state(&mut state);
        }

        assert_eq!(state, [0x233f3c9d,0x5a385202,0x56e043c9,0x76b46859]);
    }
}
