use std::cmp::Reverse;

pub fn encode(e: u128, n: u128, text: &str) {
    println!("encode");
    let bytes= text.as_bytes();
    println!("{:?}", bytes);
    // println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
    let x = bytes_to_int(bytes);
    println!("{}", x);
    println!("{:?}", &int_to_bytes(x));
}

fn bytes_to_int(bytes: &[u8]) -> u128 {
    let mut x: u128 = 0;
    for byte in bytes {
        x += *byte as u128;
        x <<= 8;
    }
    x >> 8
}

fn int_to_bytes(int: u128) -> Vec<u8> {
    let mut bytes = vec![];
    let mut x = int;
    while x != 0 {
        bytes.push((x % 256) as u8);
        x >>= 8;
    }
    bytes.reverse();
    bytes
}
