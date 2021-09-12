use super::{pkx::Pkx, poke_crypto, types};
use crate::pkrd::reader::Reader;
use ::safe_transmute::TriviallyTransmutable;
use ctr::safe_transmute;

pub type Pk7Bytes = [u8; Pk7::STORED_SIZE];

pub struct Pk7 {
    data: Pk7Bytes,
}

impl Pk7 {
    pub const STORED_SIZE: usize = 232;
    pub const BLOCK_SIZE: usize = 56;

    pub fn new(data: Pk7Bytes) -> Self {
        let seed = safe_transmute::transmute_one_pedantic(&data[0..4]).unwrap();
        Self {
            data: poke_crypto::decrypt::<{ Pk7::STORED_SIZE }, { Pk7::BLOCK_SIZE }>(data, seed),
        }
    }
}

impl Reader for Pk7 {
    fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl Pkx for Pk7 {
    fn species(&self) -> types::Species {
        self.default_read::<u16>(8).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pk7Data(Pk7Bytes);

impl Default for Pk7Data {
    fn default() -> Self {
        Self([0; Pk7::STORED_SIZE])
    }
}

// This is safe because the bytes in Pk7Data can be anything
unsafe impl TriviallyTransmutable for Pk7Data {}

impl From<Pk7Data> for Pk7 {
    fn from(data: Pk7Data) -> Self {
        Self::new(data.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_EKX: Pk7Bytes = [
        0xde, 0xda, 0x09, 0x87, 0x00, 0x00, 0x4e, 0x4b, 0x96, 0x25, 0xae, 0xf6, 0x89, 0xe7, 0x20,
        0x92, 0xc7, 0xf0, 0x8b, 0xa5, 0xe2, 0x3e, 0x6e, 0xd9, 0x52, 0x2a, 0x18, 0x04, 0x3e, 0x76,
        0xec, 0x86, 0x0f, 0x3c, 0x79, 0x44, 0xca, 0x7c, 0xe4, 0xa4, 0x85, 0x05, 0x3d, 0x60, 0x71,
        0x09, 0xb4, 0x72, 0x56, 0xab, 0xc3, 0xc4, 0x6e, 0x79, 0xd1, 0x41, 0xfc, 0xe9, 0xd9, 0x61,
        0x22, 0x04, 0x0e, 0x1f, 0xe5, 0xdf, 0xca, 0xfe, 0x57, 0x58, 0x6e, 0xcc, 0xd7, 0x81, 0xa1,
        0xf8, 0xcb, 0xf5, 0x57, 0xcd, 0xb8, 0x30, 0xbf, 0xd1, 0xe2, 0xd9, 0xb8, 0x8f, 0x79, 0x20,
        0x8c, 0x2e, 0x28, 0x50, 0x01, 0xeb, 0xe1, 0x86, 0xb5, 0x34, 0x8a, 0xfb, 0x10, 0x85, 0x1f,
        0xc6, 0xce, 0x36, 0x0f, 0x6f, 0xf2, 0xd6, 0x23, 0x06, 0x12, 0xaa, 0x75, 0xce, 0xce, 0xe0,
        0x95, 0xf3, 0xd5, 0x0f, 0x96, 0xe0, 0x44, 0x22, 0x57, 0x89, 0xfe, 0xaf, 0xda, 0x27, 0x53,
        0xa0, 0x61, 0xd2, 0x6a, 0x5a, 0xd2, 0x4d, 0xaf, 0x50, 0x0a, 0xec, 0x8c, 0x31, 0xb7, 0x48,
        0x35, 0x56, 0x3d, 0xeb, 0x93, 0xd5, 0xda, 0xed, 0xc1, 0x17, 0x5d, 0x1a, 0xce, 0xf2, 0xa8,
        0xa9, 0xc1, 0xc6, 0x41, 0xf7, 0x91, 0x38, 0x80, 0x4f, 0xf7, 0x17, 0x61, 0x1a, 0x68, 0x62,
        0xc0, 0x4c, 0x7d, 0xc4, 0x4f, 0x58, 0xe7, 0x89, 0x72, 0xae, 0x09, 0x17, 0x17, 0xa2, 0x36,
        0x01, 0xae, 0x36, 0x72, 0x09, 0x0a, 0xcc, 0xc6, 0xd4, 0xa1, 0xe6, 0x72, 0xb6, 0x65, 0xb7,
        0x79, 0x5c, 0x5b, 0x88, 0xbb, 0x23, 0xc8, 0x8d, 0x3a, 0x81, 0xd3, 0x2f, 0xf1, 0x86, 0x1d,
        0x0f, 0xa9, 0x96, 0xc6, 0x30, 0xbf, 0x71,
    ];

    #[test]
    fn should_decrypt() {
        let result: Pk7Bytes = [
            0xde, 0xda, 0x09, 0x87, 0x00, 0x00, 0x4e, 0x4b, 0xd8, 0x02, 0x00, 0x00, 0x55, 0x0d,
            0x14, 0x96, 0x30, 0x01, 0x00, 0x00, 0x43, 0x01, 0x00, 0x00, 0xb4, 0x93, 0x7a, 0xe9,
            0x0c, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x00, 0x6f, 0x00, 0x70, 0x00,
            0x70, 0x00, 0x6c, 0x00, 0x69, 0x00, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x37, 0x00, 0x2d, 0x00, 0x00, 0x00,
            0x23, 0x19, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x8e, 0x2a, 0x55, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x00, 0x56, 0x00, 0x92, 0xe0,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x12, 0x08, 0x16, 0x00, 0x00, 0x00, 0x08, 0x00, 0x04, 0x85, 0x00, 0x21,
            0x31, 0x0b, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00,
        ];

        let pkx = Pk7::new(TEST_EKX);
        assert_eq!(pkx.get_data(), result);
    }

    #[test]
    fn should_read_species() {
        let pkx = Pk7::new(TEST_EKX);
        assert_eq!(pkx.species(), types::Species::Popplio);
    }

    #[test]
    fn pk7_data_size_should_be_232() {
        assert_eq!(core::mem::size_of::<Pk7Data>(), Pk7::STORED_SIZE);
    }
}