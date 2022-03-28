use std::error::Error;
use rand::Rng;
use crate::cipher::aes_128_cbc;
use crate::cipher::aes_128_ecb;

#[derive(Debug)]
pub enum EncryptionKind {
    AesCbc,
    AesEcb,
}

#[derive(Debug)]
pub struct OracleResult {
    pub encrypted: Vec<u8>,
    pub kind: EncryptionKind,
}

pub fn encryption_oracle(input: &[u8]) -> Result<OracleResult, Box<dyn Error>> {
    // choose CBC or EBC
    let kind: EncryptionKind;
    let random = rand::random::<u8>();
    if random % 2 == 0 {
        kind = EncryptionKind::AesCbc;
    } else {
        kind = EncryptionKind::AesEcb;
    }

    // generate a random AES key and IV
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();

    // generate random bytes to prepend and append
    let prefix_num = rng.gen_range(1..6);
    let suffix_num = rng.gen_range(1..6);
    let prefix: Vec<u8> = (0..prefix_num).map(|_| rng.gen()).collect();
    let suffix: Vec<u8> = (0..suffix_num).map(|_| rng.gen()).collect();
    
    // combine all the data
    let data = [prefix, input.to_vec(), suffix].concat();

    // encrypt
    let encrypted: Vec<u8> = match kind {
        EncryptionKind::AesCbc => aes_128_cbc::encrypt(&data, &key, &iv)?,
        EncryptionKind::AesEcb => aes_128_ecb::encrypt(&data, &key, None)?,
    };

    Ok(OracleResult{
        kind: kind,
        encrypted: encrypted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::codebreak::ecb;

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("data/set2/challenge11.txt").unwrap();
        for _ in 0..1000 {
            let result = encryption_oracle(input.as_bytes()).unwrap();
            let is_ecb = ecb::is_ecb(&result.encrypted, &16);
            match result.kind {
                EncryptionKind::AesCbc => assert_eq!(false, is_ecb),
                EncryptionKind::AesEcb => assert_eq!(true, is_ecb),
            }
        }
    }
}
