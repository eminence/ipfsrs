#![feature(step_by)]

extern crate rust_base58;
extern crate rust_multihash;
extern crate protobuf;
    
use std::path::PathBuf;
use rust_base58::base58::ToBase58;
use rust_base58::FromBase58;

pub mod unixfs;
pub mod merkledag;

pub fn bin_to_hex(bin: &Vec<u8>) -> String {
    use std::borrow::Borrow;

    let mut s = String::new();
    for byte in bin {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

// Takes a hex-encoded string (like a sha256 hash) and returns a vector or bytes
pub fn hex_to_bin(hex: &str) -> Vec<u8> {

    let mut v = Vec::with_capacity(hex.len() / 2);
    for x in (0..(hex.len()-1)).step_by(2) {
        let s = &hex[x..x+2];
        let n = u8::from_str_radix(s, 16).unwrap();
        v.push(n);
    }

    v
}

pub fn build_unixfs(s: &str) -> unixfs::Data {
    let mut unix1 = unixfs::Data::new();
    unix1.set_Data(s.as_bytes().to_vec());
    unix1.set_filesize(s.len() as u64);
    unix1.set_Type(unixfs::Data_DataType::File);

    unix1
}

#[test]
fn test_hex_to_bin() {
    let s = "122070286b9afa6620a66f715c7020d68af3d10e1a497971629c07606bfdb812303d";
    let expected = vec![18, 32, 112, 40, 107, 154, 250, 102, 32, 166, 111, 113, 92, 112, 32, 214, 138, 243, 209, 14, 26, 73, 121, 113, 98, 156, 7, 96, 107, 253, 184, 18, 48, 61];
    let b = hex_to_bin(s);
    assert_eq!(b, expected);
}

#[test]
fn test_bin_to_hex() {
    let data = vec![18, 32, 112, 40, 107, 154, 250, 102, 32, 166, 111, 113, 92, 112, 32, 214, 138, 243, 209, 14, 26, 73, 121, 113, 98, 156, 7, 96, 107, 253, 184, 18, 48, 61];
    let s = bin_to_hex(&data);
    assert_eq!(s, "122070286b9afa6620a66f715c7020d68af3d10e1a497971629c07606bfdb812303d");
}

pub fn get_blockfile_from_hash(hash: &str) -> PathBuf {
    use std::env;

    let hex = bin_to_hex(&hash.from_base58().unwrap());
    let mut ipfs_path: PathBuf = env::var("IPFS_PATH").and_then(|p| Ok(PathBuf::from(p))).unwrap_or_else(|_| { env::home_dir().unwrap().join(".ipfs")});

    ipfs_path.push("blocks");
    ipfs_path.push(&hex[0..8]);
    ipfs_path.push(&hex);
    ipfs_path.set_extension("data");

    ipfs_path
}


#[test]
fn testmain() {
    use unixfs::Data_DataType;
    use rust_multihash::{multihash, HashTypes};
    use protobuf::core::Message;
    use protobuf::repeated::RepeatedField;
    

    // create a unixfs node with a "hello world" data
    let unix1 = unixfs::Data::new();
    unix1.set_Data("hello world".as_bytes().to_vec());

}


#[test]
fn test_oldmain_old() {
    use rust_multihash::{multihash, HashTypes};
    use protobuf::core::Message;

    let mut block = merkledag::PBNode::new();
    block.set_Data("hello world".as_bytes().to_vec());

    let msg: Vec<u8> = block.write_to_bytes().unwrap();
    println!("Msg len is {}", msg.len());

    // compute multihash of this data
    let mh = multihash(HashTypes::SHA2256, msg).unwrap();

    let encoded = mh.to_base58();
    println!("Hash of this object is {}", encoded);

    
}


#[test]
fn it_works() {
    use rust_base58::FromBase58;

    let s = "QmVtU7ths96fMgZ8YSZAbKghyieq7AjxNdcqyVzxTt3qVe";
    let decoded = s.from_base58().unwrap();
    assert_eq!(bin_to_hex(&decoded), "122070286b9afa6620a66f715c7020d68af3d10e1a497971629c07606bfdb812303d");
    println!("Decoded: {:?}", decoded);
}
