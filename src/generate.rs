extern crate rand;
use rand::Rng;

pub fn generate() {
    println!("{:0>1$b}", generate_integer(), 128);
    println!("generate");
}

fn generate_integer() -> u128 {
    let mut x: i128 = rand::thread_rng().gen();
    x = if x < 0 {x} else {-x};
    let y: u128 = x as u128;
    if y % 2 == 0 {y + 1} else {y}
}
