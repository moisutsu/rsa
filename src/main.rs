use clap::Clap;
use rsa::Opt;
use std::ops::*;

#[allow(clippy::clippy::many_single_char_names)]
fn main() {
    let opt = Opt::parse();

    let p = opt.p;
    let q = opt.q;
    let n = p * q;
    let l = (p - 1) * (q - 1);
    let m = opt.plaintext;
    println!("Plaintext = {}", m);
    println!("p = {}, q = {}", p, q);

    let e = generate_e(l);
    let d = generate_d(l, e);
    println!("EncryptionKey = (e: {}, n: {})", e, n);
    println!("DecryptionKey = (d: {}, n: {})", d, n);

    let c = encode(m, e, n);
    println!("Cryptogram = {}", c);

    let m = decode(c, d, n);
    println!("DecryptedText = {}", m);
}

macro_rules! mod_pow {
    ($ a : expr , $ n : expr , $ mod : expr ) => {{
        let mut ret = 1;
        let mut a = $a;
        let mut n = $n;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * a % $mod;
            }
            a = a * a % $mod;
            n >>= 1;
        }
        ret
    }};
}

fn encode(m: u128, e: u128, n: u128) -> u128 {
    mod_pow!(m, e, n)
}

fn decode(c: u128, d: u128, n: u128) -> u128 {
    mod_pow!(c, d, n)
}

fn generate_d(l: u128, e: u128) -> u128 {
    for i in 1.. {
        if (i * l + 1) % e == 0 {
            return (i * l + 1) / e;
        }
    }
    unreachable!()
}

fn generate_e(l: u128) -> u128 {
    for i in 2..l - 1 {
        if gcd(i, l) == 1 {
            return i;
        }
    }
    unreachable!()
}

fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Default + Ord + Sub<Output = T> + Rem<Output = T>,
{
    let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };
    let zero = T::default();
    while b > zero {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
