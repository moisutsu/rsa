use crate::bytes;

pub fn encode(e: u128, n: u128, text: &str) {
    println!("encode");
    let bytes= text.as_bytes();
    println!("{:?}", bytes);
    // println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
    let x = bytes::to_int(bytes);
    println!("{}", x);
    println!("{:?}", &bytes::from_int(x));
}
