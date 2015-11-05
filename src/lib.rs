#![feature(step_by)]
#![feature(path_ext_deprecated)]

extern crate rust_base58;
extern crate rust_multihash;
extern crate protobuf;
extern crate rustc_serialize;
//extern crate asn1;
//extern crate num;
extern crate openssl;

use std::path::PathBuf;
use std::cell::RefCell;

use rust_base58::base58::ToBase58;
use rust_base58::FromBase58;
use rust_multihash::{multihash, HashTypes};
use protobuf::core::Message;
use rustc_serialize::base64::{ToBase64,FromBase64, Config, CharacterSet, Newline};

pub mod unixfs;
pub mod merkledag;
pub mod dht;
pub mod crypto;

pub mod multihash;

use multihash::*;


/// A wrapper around a PBNode and multihash
///
/// Given a PBNode, you sometimes want to operate on the node object itself, and other times you
/// just want to print or use the hash.  Instead of recalculating the hash every time, calc it once
/// and then keep them together
///
#[derive(Debug)]
pub struct Node {
    node: Option<merkledag::PBNode>,
    pub mh_bytes: MultihashBytes,
    pub mh_str: MultihashStr,
}

impl Node {
    /// Calculates hash from an actual object
    pub fn from_pb(node: merkledag::PBNode) -> Node {
        
        let msg: Vec<u8> = node.write_to_bytes().unwrap();
        let mh = multihash(HashTypes::SHA2256, msg).unwrap();
        let mh_str = mh.to_base58();

        Node{ node: Some(node), mh_bytes: multihash::MultihashBytes(mh), mh_str: multihash::MultihashStr(mh_str)}
    }

    pub fn from_mh<M: Multihash>(m: M) -> Node {
        Node{node: None, mh_bytes: MultihashBytes(m.as_bytes()), mh_str: MultihashStr(m.base58())}

    }

    /// Load the given object from disk
    pub fn load_from_disk(&mut self) {
        use std::io::Read;
        if self.node.is_none() {
            // load the object from disk
            let obj_path = get_blockfile_from_hash(&self.mh_bytes);
            let mut f = std::fs::File::open(&obj_path).unwrap();
            let mut b = Vec::new();
            f.read_to_end(&mut b);
            let mut pbn = merkledag::PBNode::new();
            pbn.merge_from_bytes(&b);
            self.node = Some(pbn);
        }
    }

    pub fn get_node(&self) -> &merkledag::PBNode {
        if let Some(ref n) = self.node {
            n
        } else {
            panic!("This node should be loaded from disk first");
        }
    }
}

impl AsRef<multihash::MultihashBytes> for Node {
    fn as_ref(&self) -> &multihash::MultihashBytes {
        &self.mh_bytes
    }
}

impl AsRef<multihash::MultihashStr> for Node {
    fn as_ref(&self) -> &multihash::MultihashStr {
        &self.mh_str
    }
}

impl AsRef<merkledag::PBNode> for Node {
    fn as_ref(&self) -> &merkledag::PBNode {
        self.get_node()
    }
}
    

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

/// Given a hex-encoded hash, return the IPFS_PATH that should contain this data
pub fn get_blockfile_from_hash<M: multihash::Multihash>(hash: M) -> PathBuf {
    use std::env;

    let hex = hash.as_hex();
    let mut ipfs_path: PathBuf = env::var("IPFS_PATH").and_then(|p| Ok(PathBuf::from(p))).unwrap_or_else(|_| { env::home_dir().unwrap().join(".ipfs")});

    ipfs_path.push("blocks");
    ipfs_path.push(&hex[0..8]);
    ipfs_path.push(&hex);
    ipfs_path.set_extension("data");

    ipfs_path
}

/// Writes a PBNode to disk to ~/.ipfs/blocks
pub fn write_node_to_disk(node: &Node) {
    use std::fs::PathExt;

    let pbnode = node.get_node();
    let msg: Vec<u8> = pbnode.write_to_bytes().unwrap();

    let path = get_blockfile_from_hash(&node.mh_bytes);

    {
        let d = path.parent().unwrap();
        if !d.exists() {
            std::fs::create_dir(&d).unwrap();
        }

        let mut f = std::fs::File::create(&path).unwrap();
        pbnode.write_to_writer(&mut f).unwrap();
    }

}

/// Given a base64-encoded key from an ipfs config file, produce a PKey
pub fn read_privkey(privkey_str: &str) -> openssl::crypto::pkey::PKey {
    let bytes = privkey_str.from_base64().unwrap();
    let mut privkey = crypto::PrivateKey::new();
    privkey.merge_from_bytes(&bytes).unwrap();

    let privkey_bytes: Vec<u8> = privkey.take_Data();
    let mut pkey = openssl::crypto::pkey::PKey::new();
    pkey.load_priv(&privkey_bytes);
    pkey
}

pub fn get_pubkey_id(pkey: openssl::crypto::pkey::PKey) {
    let pubkey = pkey.save_pub();
    let mut pubkey_pb = crypto::PublicKey::new();
    pubkey_pb.set_Data(pubkey);
    pubkey_pb.set_Type(crypto::KeyType::RSA);
    let pubkey_bytes = pubkey_pb.write_to_bytes().unwrap();
    let pubkey_mh = multihash(HashTypes::SHA2256, pubkey_bytes).unwrap();
    let pubkey_encoded = &pubkey_mh.to_base58();
}

#[test]
fn testmain() {

}


#[test]
fn test_oldmain_old() {
    use rust_multihash::{multihash, HashTypes};

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
