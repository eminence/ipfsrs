extern crate ipfsrs;
extern crate rust_base58;
extern crate rust_multihash;

use ipfsrs::hex_to_bin;
use std::env;
use rust_multihash::HashTypes;
use rust_base58::ToBase58;

fn hex2mh(input: &str) -> String {
    let mut bin = hex_to_bin(input);
    let bin_len = bin.len() as u8;
    let hash = HashTypes::SHA2256.to_u8();
    bin.insert(0, bin_len);
    bin.insert(0, hash);
    bin.to_base58()
}

fn main() {
    let input = env::args().nth(1).unwrap();
    let mh = hex2mh(input.trim());
    println!("{}", mh);


}
