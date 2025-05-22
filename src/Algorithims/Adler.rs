pub fn Adler_32(datastream: &[u8]) -> u32 {
    let mut sum_1 = 1;
    let mut sum_2 = 0;
    for i in datastream {
        sum_1 = (sum_1 + (*i as u32)) % 65521;
        sum_2 = (sum_1 + sum_2) % 65521;
    }

    (sum_2 << 16) | sum_1
}   