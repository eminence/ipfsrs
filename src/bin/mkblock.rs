#![feature(path_ext_deprecated)]

extern crate ipfsrs;
extern crate rust_base58;
extern crate rust_multihash;
extern crate protobuf;

use ipfsrs::*;
use rust_multihash::{multihash, HashTypes};
use protobuf::core::Message;
use protobuf::repeated::RepeatedField;
use rust_base58::ToBase58;
use std::fs::File;


// returns the hash of the object written
fn write_data_to_fs(data: &merkledag::PBNode) -> Vec<u8> {
    use std::path::PathBuf;
    use std::fs::PathExt;
    let bytes = data.write_to_bytes().unwrap();
    let mh = multihash(HashTypes::SHA2256, bytes.clone()).unwrap();
    let hash = bin_to_hex(&mh);

    let prefix = &hash[0..8];
    let mut block_dir = PathBuf::from("/storage/home/achin/.ipfs/blocks");
    block_dir.push(prefix);
    if !block_dir.exists() {
        std::fs::create_dir(&block_dir);
    }

    block_dir.push(&hash);
    block_dir.set_extension("data");
    let mut f = File::create(&block_dir).unwrap();
    data.write_to_writer(&mut f);

    println!("Data written: {}", mh.to_base58());

    mh

}

fn build_node_with_data(data: &unixfs::Data) -> merkledag::PBNode {

    let mut m = merkledag::PBNode::new();
    m.set_Data(data.write_to_bytes().unwrap());
    m
}

    
fn main() {
    // create a unixfs node with a "hello world" data
    let unix1 = build_unixfs("hello world");
    let unix1_bytes = unix1.write_to_bytes().unwrap();

    // create a merklenode with this data
    let merkle1 = build_node_with_data(&unix1);
    println!("Writing example hash 1:");
    write_data_to_fs(&merkle1);


    // create a unixfs node with a "hello " data, and another rnode with a "world" data
    let unix2 = build_node_with_data(&build_unixfs("hello "));
    let unix2_hash = write_data_to_fs(&unix2);

    let unix3 = build_node_with_data(&build_unixfs("world"));
    let unix3_hash = write_data_to_fs(&unix3);
    
    let mut merkle2 = merkledag::PBNode::new();

    let mut links: Vec<merkledag::PBLink> = Vec::new();
    links.push( {
        let mut lnk = merkledag::PBLink::new();
        lnk.set_Hash(unix2_hash.clone());
        lnk.clear_Name();
        lnk.set_Tsize(unix2.write_to_bytes().unwrap().len() as u64);
        lnk
    });
    links.push( {
        let mut lnk = merkledag::PBLink::new();
        lnk.set_Hash(unix3_hash.clone());
        lnk.clear_Name();
        lnk.set_Tsize(unix3.write_to_bytes().unwrap().len() as u64);
        lnk
    });

    let repeated = protobuf::repeated::RepeatedField::from_vec(links);
    merkle2.set_Links(repeated);
    let mut empty_unixfs = build_unixfs("");
    empty_unixfs.clear_Data();
    empty_unixfs.set_filesize(11);
    empty_unixfs.set_blocksizes( {
        let mut v = Vec::new();
        v.push(6);
        v.push(5);
        v

    });
    merkle2.set_Data(empty_unixfs.write_to_bytes().unwrap());
    
    { 
    let bytes = merkle2.write_to_bytes().unwrap();
    let new_mh = multihash(HashTypes::SHA2256, bytes).unwrap();
    println!("Calculated multihash for this object: {}", new_mh.to_base58());
    println!("Written to {}", bin_to_hex(&new_mh));

    write_data_to_fs(&merkle2);
    }

}
