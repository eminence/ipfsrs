extern crate ipfsrs;
extern crate rust_base58;

use rust_base58::ToBase58;
use ipfsrs::hex_to_bin;
use std::env;

fn main() {
    let input = env::args().nth(1).unwrap();
    let mut bin = hex_to_bin(&input);

    println!("{}", bin.to_base58());


}
