extern crate rsa;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("rsa")
        .version("1.0")
        .author("j-hira")
        .about("RSA encryption tool.")
        .arg(Arg::with_name("encode")
            .short("e")
            .long("encode")
            .value_name("String")
            .requires("key")
            .conflicts_with_all(&["decode", "generate"]))
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .value_name("String")
            .requires("key")
            .conflicts_with("generate"))
        .arg(Arg::with_name("key")
            .short("k")
            .long("key")
            .value_name("Key")
            .conflicts_with("generate"))
        .arg(Arg::with_name("generate")
            .short("g")
            .long("generate"))
        .get_matches();

    if let Some(v) = matches.value_of("encode") {
        rsa::encode();
        println!("key: {}", v);
    } else if let Some(v) = matches.value_of("decode") {
        rsa::decode();
        println!("key: {}", v);
    } else if matches.is_present("generate") {
        rsa::generate();
    } else {
        println!("Input arguments!");
    }
}
