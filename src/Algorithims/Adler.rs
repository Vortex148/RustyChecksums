pub fn Fletcher_16(datastream:&[u8]) -> u16 {
    let mut sum_1: u16 = 0;
    let mut sum_2: u16 = 0;

    for i in datastream{
        sum_1 = (sum_1 + *i as u16) % 0x0000_00FF;
        sum_2 = (sum_2 + sum_1) % 0x0000_00FF;
    }

    (sum_2 << 8) | sum_1
}

pub fn Fletcher_32(datastream:&[u8]) -> u32 {
    let mut sum_1: u32 = 0;
    let mut sum_2: u32 = 0;

    for &i in datastream{
        sum_1 = (sum_1 + i as u32) % 0x0000_FFFF;
        sum_2 = (sum_2 + sum_1) % 0x0000_FFFF;
    }

    (sum_2 << 16) | sum_1
}

pub fn Fletcher_64(datastream:&[u8]) -> u64 {
    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;

    for &i in datastream{
        sum_1 = (sum_1 + i as u64) % 0xFFFF_FFFF;
        sum_2 = (sum_2 + sum_1) % 0xFFFF_FFFF;
    }

    (sum_2 << 32) | sum_1
}