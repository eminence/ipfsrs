extern crate ipfsrs;
extern crate openssl;
extern crate rustc_serialize;
extern crate rust_base58;
extern crate rust_multihash;
extern crate protobuf;

use rustc_serialize::base64::{ToBase64,FromBase64, Config, CharacterSet, Newline};
use rust_multihash::{multihash, HashTypes};
use rust_base58::base58::ToBase58;
use protobuf::core::Message;

use ipfsrs::crypto;

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::io::Write;

static key_count: AtomicUsize = ATOMIC_USIZE_INIT;
const MAX_KEYS: usize = 5000;


fn run() {
    use openssl::crypto::pkey::PKey;
    let wants = vec!();

    loop {
        let mut pkey = PKey::new();
        pkey.gen(2048);

        //println!("Pubkey: {}", pubkey_encoded);

        let mut pubkey_pb = crypto::PublicKey::new();
        pubkey_pb.set_Data(pkey.save_pub());
        pubkey_pb.set_Type(crypto::KeyType::RSA);
        let pubkey_bytes = pubkey_pb.write_to_bytes().unwrap();
        let pubkey_mh = multihash(HashTypes::SHA2256, pubkey_bytes).unwrap();
        let peerid = pubkey_mh.to_base58();
        let short = peerid.to_lowercase();
        if wants.iter().any(|x| short.contains(x)) {
            println!("PeerID: \"{}\"", pubkey_mh.to_base58());

            let mut seckey_pb = crypto::PrivateKey::new();
            seckey_pb.set_Data(pkey.save_priv());
            seckey_pb.set_Type(crypto::KeyType::RSA);
            let seckey_bytes = seckey_pb.write_to_bytes().unwrap();
            //let seckey_mh = multihash(HashTypes::SHA2256, seckey_bytes.clone()).unwrap();
            let cfg = Config{char_set: CharacterSet::Standard, newline: Newline::LF, pad: true, line_length: None};
            let seckey_encoded = &seckey_bytes.to_base64(cfg);
            println!("PrivKey: \"{}\"", seckey_encoded);
            if let Ok(mut f) = std::fs::File::create(&peerid) {
                f.write_all(seckey_encoded.as_bytes());
            }
        }

        let count = key_count.fetch_add(1, Ordering::Relaxed);
        if count > MAX_KEYS {
            return;
        } 
    }


    //println!("PeerID: \"{}\"", peerid);
}

fn main() {
    use std::thread;

    let mut threads = Vec::new();
    threads.push(thread::spawn(|| run()));
    threads.push(thread::spawn(|| run()));
    threads.push(thread::spawn(|| run()));
    threads.push(thread::spawn(|| run()));
    threads.push(thread::spawn(|| run()));
    threads.push(thread::spawn(|| run()));
    loop {
        thread::sleep_ms(10000);
        let count = key_count.load(Ordering::Relaxed);
        println!("Keys: {}", count);
        if count > MAX_KEYS { break; }
    }

    for t in threads {
        t.join().unwrap();
    }

}
