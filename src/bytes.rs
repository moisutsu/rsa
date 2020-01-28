pub fn to_int(bytes: &[u8]) -> u128 {
    let mut x: u128 = 0;
    for byte in bytes {
        x += *byte as u128;
        x <<= 8;
    }
    x >> 8
}

pub fn from_int(int: u128) -> Vec<u8> {
    let mut bytes = vec![];
    let mut x = int;
    while x != 0 {
        bytes.push((x % 256) as u8);
        x >>= 8;
    }
    bytes.reverse();
    bytes
}
