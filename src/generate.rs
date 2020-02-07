extern crate rand;
use rand::Rng;

pub fn generate() {
    let p = generate_prime();
    let q = generate_prime();
    println!("p={}, q={}", p, q);
    let n = p * q;
    let l = lcm(p - 1, q - 1);
    let e = generate_e(l);
    let d = generate_d(l, e);

    println!("Public key");
    println!("({}, {})", e, n);
    println!("Secret key");
    println!("({}, {})", d, n);
}

fn generate_d(l: u128, e: u128) -> u128 {
    let mut i = 1;
    loop {
        if (i * l + 1) % e == 0 {
            return (i * l + 1) / e;
        }
        i += 1;
    }
}

fn generate_e(l: u128) -> u128 {
    for i in 2..l-1 {
        if gcd(i, l) == 1 {
            return i;
        }
    }
    0
}

fn generate_prime() -> u128 {
    let mut x;
    loop {
        x = generate_integer();
        if is_prime(x) {
            break;
        }
    }
    x as u128
}

fn generate_integer() -> u8 {
    let mut x: i8 = rand::thread_rng().gen();
    x = if x < 0 {x} else {-x};
    let y: u8 = x as u8;
    if y % 2 == 0 {y + 1} else {y}
}

fn is_prime(num: u8) -> bool {
    if num == 2 {
        return true
    }
    if num < 2 || num & 1 == 0 {
        return false
    }
    let upper_limit = (num as f64).sqrt() as u8 + 1;
    for i in (3..upper_limit).step_by(2) {
        if num % i == 0 {
            return false
        }
    }
    true
}

fn gcd(p: u128, q: u128) -> u128 {
    let (mut a, mut b) = if p > q {(p, q)} else {(q, p)};
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }
    b
}

fn lcm(p: u128, q: u128) -> u128 {
    p * q / gcd(p, q)
}
