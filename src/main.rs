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
            .value_name("Key")
            .requires_all(&["n", "string"])
            .conflicts_with_all(&["decode", "generate"])
            .required_unless_one(&["decode", "generate"]))
        .arg(Arg::with_name("decode")
            .short("d")
            .long("decode")
            .value_name("Key")
            .requires_all(&["n", "string"])
            .conflicts_with_all(&["encode", "generate"])
            .required_unless_one(&["encode", "generate"]))
        .arg(Arg::with_name("n")
            .short("n")
            .value_name("Key")
            .conflicts_with("generate"))
        .arg(Arg::with_name("string")
            .short("s")
            .long("string")
            .value_name("Text")
            .conflicts_with("generate"))
        .arg(Arg::with_name("generate")
            .short("g")
            .long("generate"))
        .get_matches();

    if let Some(e) = matches.value_of("encode") {
        let n: u128 = matches.value_of("n").unwrap().parse().unwrap();
        let text = matches.value_of("string").unwrap();
        rsa::encode(e.parse().unwrap(), n, text);
    } else if let Some(d) = matches.value_of("decode") {
        let n = matches.value_of("n").unwrap().parse().unwrap();
        let text = matches.value_of("string").unwrap();
        rsa::decode(d.parse().unwrap(), n, text);
    } else if matches.is_present("generate") {
        rsa::generate();
    }
}
