extern crate ipfsrs;
extern crate rust_base58;
extern crate rust_multihash;

use ipfsrs::bin_to_hex;
use std::env;
use rust_multihash::HashTypes;
use rust_base58::FromBase58;


fn main() {
    let input = env::args().nth(1).unwrap();
    let bytes: Vec<u8> = input.from_base58().unwrap();
    let hash_type = HashTypes::from_u8(bytes[0]).unwrap();
    println!("Multihash type: {:?}", hash_type);
    println!("Hash: {}", bin_to_hex(&bytes));

}
