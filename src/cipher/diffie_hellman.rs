use num_bigint::{BigUint, RandBigInt};
use openssl::sha::sha256;

#[derive(Debug, PartialEq)]
pub struct SharedKey {
    pub key: Vec<u8>,
    pub hmac: Vec<u8>
}

pub fn new_private_key(key_size_bits: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_biguint(key_size_bits)
}

pub fn new_public_key(private_key: &BigUint, g: &BigUint, p: &BigUint) -> BigUint {
    (g ^ private_key) % p
}

pub fn generate_shared_key(private_key: &BigUint, public_key: &BigUint, g: &BigUint) -> SharedKey {
    let s = (public_key ^ private_key) % g;
    let hashed = sha256(s.to_bytes_be().as_ref());
    SharedKey {
        key: hashed[..16].to_vec(),
        hmac: hashed[16..].to_vec()
    }
}