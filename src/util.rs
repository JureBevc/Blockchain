pub fn u16_bytes(x : &u16) -> [u8; 2]{
    return [
        (x >> (8 * 0)) as u8,
        (x >> (8 * 1)) as u8
    ];
}

pub fn u32_bytes(x : &u32) -> [u8; 4]{
    return [
        (x >> (8 * 0)) as u8,
        (x >> (8 * 1)) as u8,
        (x >> (8 * 2)) as u8,
        (x >> (8 * 3)) as u8
    ];
}

pub fn u64_bytes(x : &u64) -> [u8; 8]{
    return [
        (x >> (8 * 0)) as u8,
        (x >> (8 * 1)) as u8,
        (x >> (8 * 2)) as u8,
        (x >> (8 * 3)) as u8,
        (x >> (8 * 4)) as u8,
        (x >> (8 * 5)) as u8,
        (x >> (8 * 6)) as u8,
        (x >> (8 * 7)) as u8
    ];
}

pub fn u128_bytes(x : &u128) -> [u8; 16]{
    return [
        (x >> (8 * 0)) as u8,
        (x >> (8 * 1)) as u8,
        (x >> (8 * 2)) as u8,
        (x >> (8 * 3)) as u8,
        (x >> (8 * 4)) as u8,
        (x >> (8 * 5)) as u8,
        (x >> (8 * 6)) as u8,
        (x >> (8 * 7)) as u8,
        (x >> (8 * 8)) as u8,
        (x >> (8 * 9)) as u8,
        (x >> (8 * 10)) as u8,
        (x >> (8 * 11)) as u8,
        (x >> (8 * 12)) as u8,
        (x >> (8 * 13)) as u8,
        (x >> (8 * 14)) as u8,
        (x >> (8 * 15)) as u8
    ];
}

pub fn hash_to_difficulty_bytes(hash: &Vec<u8>) -> u64{
    ((hash[0] as u64) << 8*7) |
    ((hash[1] as u64) << 8*6) |
    ((hash[2] as u64) << 8*5) |
    ((hash[3] as u64) << 8*4) |
    ((hash[4] as u64) << 8*3) |
    ((hash[5] as u64) << 8*2) |
    ((hash[6] as u64) << 8*1) |
    ((hash[7] as u64) << 8*0)
}