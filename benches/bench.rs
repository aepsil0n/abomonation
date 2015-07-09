#![feature(test)]

extern crate abomonation;
extern crate test;

use abomonation::*;
use test::Bencher;
use std::io::Read;

#[bench] fn enc_empty(bencher: &mut Bencher) { _bench_enc(bencher, vec![(); 1024]); }
#[bench] fn dec_empty(bencher: &mut Bencher) { _bench_dec(bencher, vec![(); 1024]); }

#[bench] fn enc_u64(bencher: &mut Bencher) { _bench_enc(bencher, vec![0u64; 1024]); }
#[bench] fn dec_u64(bencher: &mut Bencher) { _bench_dec(bencher, vec![0u64; 1024]); }

#[bench] fn enc_u8_u64(bencher: &mut Bencher) { _bench_enc(bencher, vec![(0u8, 0u64); 512]); }
#[bench] fn dec_u8_u64(bencher: &mut Bencher) { _bench_dec(bencher, vec![(0u8, 0u64); 512]); }

#[bench] fn enc_string10(bencher: &mut Bencher) { _bench_enc(bencher, vec![format!("grawwwwrr!"); 1024]); }
#[bench] fn dec_string10(bencher: &mut Bencher) { _bench_dec(bencher, vec![format!("grawwwwrr!"); 1024]); }

#[bench] fn enc_string20(bencher: &mut Bencher) { _bench_enc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]); }
#[bench] fn dec_string20(bencher: &mut Bencher) { _bench_dec(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]); }

#[bench] fn enc_vec_u_s(bencher: &mut Bencher) { _bench_enc(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn dec_vec_u_s(bencher: &mut Bencher) { _bench_dec(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn own_vec_u_s(bencher: &mut Bencher) { _bench_own(bencher, vec![vec![(0u64, format!("grawwwwrr!")); 32]; 32]); }

#[bench] fn enc_vec_u_vn_s(bencher: &mut Bencher) { _bench_enc(bencher, vec![vec![(0u64, vec![(); 1 << 40], format!("grawwwwrr!")); 32]; 32]); }
#[bench] fn dec_vec_u_vn_s(bencher: &mut Bencher) { _bench_dec(bencher, vec![vec![(0u64, vec![(); 1 << 40], format!("grawwwwrr!")); 32]; 32]); }

fn _bench_enc<T: Abomonation>(bencher: &mut Bencher, vector: Vec<T>) {

    // prepare encoded data for bencher.bytes
    let mut bytes = Vec::new();
    encode(&vector, &mut bytes);

    // repeatedly encode this many bytes
    bencher.bytes = bytes.len() as u64;
    bencher.iter(|| {
        bytes.clear();
        encode(&vector, &mut bytes);
    });
}

fn _bench_dec<T: Abomonation+Eq>(bencher: &mut Bencher, vector: Vec<T>) {

    // prepare encoded data
    let mut bytes = Vec::new();
    encode(&vector, &mut bytes);

    // repeatedly decode (and validate)
    bencher.bytes = bytes.len() as u64;
    bencher.iter(|| {
        let result = decode::<Vec<T>>(&mut bytes).unwrap();
        assert!(result.len() == vector.len());
        for i in 0..result.len() {
            assert!(result[i] == vector[i]);
        }
    });
}

fn _bench_own<T: Abomonation+Eq+Clone>(bencher: &mut Bencher, vector: Vec<T>) {

    // prepare encoded data
    let mut bytes = Vec::new();
    encode(&vector, &mut bytes);

    // repeatedly decode (and validate)
    bencher.bytes = bytes.len() as u64;
    bencher.iter(|| {
        let result = (*decode::<Vec<T>>(&mut bytes[..]).unwrap()).to_vec();
        assert!(result.len() == vector.len());
        for i in 0..result.len() {
            assert!(result[i] == vector[i]);
        }
    });
}
