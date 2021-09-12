// Thanks to PKHeX - https://github.com/kwsch/PKHeX/blob/4bb54334899cb2358b66bf97ba8d7f59c22430d7/PKHeX.Core/PKM/Util/PokeCrypto.cs

#[rustfmt::skip]
const BLOCK_POSITION: [usize; 128] = [
    0, 1, 2, 3,
    0, 1, 3, 2,
    0, 2, 1, 3,
    0, 3, 1, 2,
    0, 2, 3, 1,
    0, 3, 2, 1,
    1, 0, 2, 3,
    1, 0, 3, 2,
    2, 0, 1, 3,
    3, 0, 1, 2,
    2, 0, 3, 1,
    3, 0, 2, 1,
    1, 2, 0, 3,
    1, 3, 0, 2,
    2, 1, 0, 3,
    3, 1, 0, 2,
    2, 3, 0, 1,
    3, 2, 0, 1,
    1, 2, 3, 0,
    1, 3, 2, 0,
    2, 1, 3, 0,
    3, 1, 2, 0,
    2, 3, 1, 0,
    3, 2, 1, 0,

    // duplicates of 0-7 to eliminate modulus
    0, 1, 2, 3,
    0, 1, 3, 2,
    0, 2, 1, 3,
    0, 3, 1, 2,
    0, 2, 3, 1,
    0, 3, 2, 1,
    1, 0, 2, 3,
    1, 0, 3, 2,
];

fn crypt_pkm<const PKX_SIZE: usize>(mut data: [u8; PKX_SIZE], mut seed: u32) -> [u8; PKX_SIZE] {
    data.chunks_mut(2).skip(4).for_each(|bytes| {
        seed = 0x41c64e6du32.wrapping_mul(seed).wrapping_add(0x6073);
        bytes[0] ^= (seed >> 16) as u8;
        bytes[1] ^= (seed >> 24) as u8;
    });

    data
}

fn shuffle_array<const PKX_SIZE: usize>(
    data: [u8; PKX_SIZE],
    seed: u32,
    block_size: usize,
) -> [u8; PKX_SIZE] {
    let sv = ((seed >> 13) & 31) as usize;

    let mut result: [u8; PKX_SIZE] = [0; PKX_SIZE];
    result.copy_from_slice(&data);

    for block in 0..4 {
        let offset = BLOCK_POSITION[(sv * 4) + block];

        let source_start = 8 + (block_size * offset);
        let dest_start = 8 + (block_size * block);

        let source_block = &data[source_start..source_start + block_size];
        let dest_block = &mut result[dest_start..dest_start + block_size];

        dest_block.copy_from_slice(source_block);
    }

    result
}

pub(super) fn decrypt<const PKX_SIZE: usize, const BLOCK_SIZE: usize>(
    ekx: [u8; PKX_SIZE],
    seed: u32,
) -> [u8; PKX_SIZE] {
    let pkx = crypt_pkm(ekx, seed);
    shuffle_array(pkx, seed, BLOCK_SIZE)
}
