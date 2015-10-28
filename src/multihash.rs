
use rust_base58::FromBase58;
use rust_base58::ToBase58;
use ::bin_to_hex;

/// The base58 encoding of a multihash
#[derive(Debug)]
pub struct MultihashStr(pub String);

/// The raw multihash bytes
#[derive(Debug)]
pub struct MultihashBytes(pub Vec<u8>);

/// The hex encoding of a multihash
#[derive(Debug)]
pub struct MultihashHex(pub String);

/// 
pub trait Multihash {
    fn base58(&self) -> String;
    fn as_bytes(&self) -> Vec<u8>;
    fn as_hex(&self) -> String;
}


impl Multihash for MultihashStr {
    fn base58(&self) -> String {
        let &MultihashStr(ref s) = self; s.clone()
    }
    fn as_bytes(&self) -> Vec<u8> {
        let &MultihashStr(ref s) = self;
        s.from_base58().unwrap()
    }
    fn as_hex(&self) -> String {
        let &MultihashStr(ref s) = self;
        let bytes = s.from_base58().unwrap();
        bin_to_hex(&bytes)
    }
}

impl Multihash for MultihashBytes {
    fn base58(&self) -> String {
        let &MultihashBytes(ref b) = self;
        b.to_base58()
    }
    fn as_bytes(&self) -> Vec<u8> {
        let &MultihashBytes(ref b) = self;
        b.clone()
    }
    fn as_hex(&self) -> String {
        let &MultihashBytes(ref b) = self;
        bin_to_hex(b)
    }
}


impl<'a, T: Multihash> Multihash for &'a T {
    fn base58(&self) -> String {
       (*self).base58() 
    }
    fn as_bytes(&self) -> Vec<u8> {
        (*self).as_bytes()
    }
    fn as_hex(&self) -> String {
        (*self).as_hex()
    }
}
