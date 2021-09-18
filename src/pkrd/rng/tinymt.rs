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

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_shuffle() {
//         let mut rng = MT::new(0xaabbccdd);
//         for _ in 0..624 {
//             rng.next();
//         }

//         let result = rng.next();
//         assert_eq!(result, 0x796d251a);
//     }
// }
