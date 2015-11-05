extern crate rust_base58;
extern crate rust_multihash;
extern crate protobuf;
extern crate ipfsrs;


use ipfsrs::unixfs::Data;
use ipfsrs::merkledag::PBNode;
use ipfsrs::{get_blockfile_from_hash};
use ipfsrs::multihash::*;

use std::fs::File;
use rust_multihash::{HashTypes, multihash};
use std::io::Read;
use protobuf::core::Message;
use rust_base58::ToBase58;

fn main() {
    use std::env;

    let hash = MultihashStr(env::args().nth(1).unwrap());
    let blockfile = get_blockfile_from_hash(hash);
    println!("opening {:?}", blockfile);
    let mut f = File::open(blockfile).unwrap();
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes);

    let mut pbnode =  PBNode::new();
    pbnode.merge_from_bytes(&bytes).unwrap_or_else(|e| panic!("failed to merge from bytes: {:?}", e));
    let mut total_size: u64 = 0;
    if pbnode.has_Data() {
        println!("has data: true");
        let data = pbnode.get_Data();
        println!("data len: {} bytes", data.len());
        total_size += (data.len() as u64);

        let mut data_obj: Data = Data::new();
        if let Ok(_) = data_obj.merge_from_bytes(data) {
            println!("Decoded data as a unixfs object:");
            println!("  Type is {:?}", data_obj.get_Type());
            if data_obj.has_Data() {
                println!("  has data: {:?}", data_obj.get_Data());
            }
            if data_obj.has_filesize() {
                println!("  has filesize: {}", data_obj.get_filesize());
            }
            for bs in data_obj.get_blocksizes() {
                println!("  blocksize: {:?}", bs);
            }

        } else {
            println!("Could not be decoded as a unixfs object");
        }
    } else {
        println!("This PBNode has no data field (since it is optional)");
    }
    let links = pbnode.get_Links();
    println!("num links: {}", links.len());
    for link in links {
        let pretty_hashname = link.get_Hash().to_base58();
        let hashfile = get_blockfile_from_hash(MultihashBytes(link.get_Hash().to_vec()));
        println!("  link {} -> {:?} {}", link.get_Name(), pretty_hashname, link.get_Tsize());
        total_size += (link.get_Tsize() as u64);

    }
    println!("Total size of this block and all links: {} bytes", total_size);
    let new_mh = multihash(HashTypes::SHA2512, bytes).unwrap();
    println!("Recalculated multihash for this object: {}", new_mh.to_base58());

}
