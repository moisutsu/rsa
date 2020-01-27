extern crate rand;
use rand::Rng;

pub fn generate() {
    println!("generate");
    println!("{}", generate_prime());
}

fn generate_prime() -> u32 {
    let mut x;
    loop {
        x = generate_integer();
        if is_prime(x) {
            break;
        }
    }
    x
}

fn generate_integer() -> u32 {
    let mut x: i32 = rand::thread_rng().gen();
    x = if x < 0 {x} else {-x};
    let y: u32 = x as u32;
    if y % 2 == 0 {y + 1} else {y}
}

fn is_prime(num: u32) -> bool {
    if num == 2 {
        return true
    }
    if num < 2 || num & 1 == 0 {
        return false
    }
    let upper_limit = (num as f64).sqrt() as u32 + 1;
    for i in (3..upper_limit).step_by(2) {
        if num % i == 0 {
            return false
        }
    }
    true
}
