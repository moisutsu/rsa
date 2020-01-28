use crate::bytes;
use rug::{Integer, ops::Pow};

pub fn encode(e: u128, n: u128, text: &str) {
    let bytes = text.as_bytes();
    let x = Integer::from(bytes::to_int(bytes));
    let cryptogram = x.pow(e as u32) % n;
    println!("{}", unsafe {String::from_utf8_unchecked(bytes::from_int(integer_to_int(cryptogram)).to_vec())});
}

fn integer_to_int(x: Integer) -> u128 {
    let mut t = x;
    let mut r: u128 = 0;
    loop {
        r += if t.clone() % 2 == 1 {1} else {0};
        r <<= 1;
        t >>= 1;
        if t == 0 {break;}
    }
    r
}
