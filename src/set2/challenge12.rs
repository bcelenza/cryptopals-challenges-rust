
use base64;
use crate::cipher::aes_128_ecb;
use lazy_static::lazy_static;
use rand::Rng;
use std::error::Error;
use std::collections::HashMap;

lazy_static! {
    // create a random static key
    static ref KEY: Vec<u8> = {
        let mut rng = rand::thread_rng();
        (0..16).map(|_| rng.gen()).collect()
    };
    // create the decoded unknown string for comparison
    static ref UNKNOWN_STRING: Vec<u8> = base64::decode(String::from("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").as_bytes().to_vec()).unwrap();
}

pub fn encryption_oracle(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // append static text
    let data = [input.to_vec(), UNKNOWN_STRING.to_vec()].concat();

    // encrypt
    let encrypted = aes_128_ecb::encrypt(&data, KEY.as_ref(), None)?;
    Ok(encrypted)
}

pub fn dictionary(prefix: &[u8], block_start: usize, block_end: usize) -> HashMap<Vec<u8>, u8> {
    let mut dictionary: HashMap<Vec<u8>, u8> = HashMap::new();
    for byte in 0..u8::MAX {
        let input = [prefix.to_owned(), vec![byte]].concat();
        let block = encryption_oracle(input.as_ref()).unwrap()[block_start..block_end].to_vec();
        dictionary.insert(block, byte);
    }
    assert_eq!(u8::MAX as usize, dictionary.len());
    dictionary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codebreak::ecb_detector;
    use crate::pkcs7::pkcs7;

    #[test]
    fn test_s2c12() {
        // find block size based on when the ciphertext length changes
        let mut block_size: usize = 0;
        let mut last_ciphertext_size: usize = 0;
        for s in 1..64 {
            let input: Vec<u8> = (0..s).map(|_| 65 ).collect();
            let ciphertext = encryption_oracle(input.as_ref()).unwrap();
            if last_ciphertext_size > 0 && ciphertext.len() > last_ciphertext_size {
                block_size = ciphertext.len() - last_ciphertext_size;
                break;
            }
            last_ciphertext_size = ciphertext.len();
        }
        assert_eq!(block_size, KEY.len());

        // determine if it's ECB by feeding it a repeated block
        assert!(ecb_detector::is_ecb(encryption_oracle((0..block_size*2).map(|_| 65).collect::<Vec<u8>>().as_ref()).unwrap().as_ref(), &block_size));

        // decrypt one block, one byte at a time
        let mut message: Vec<u8> = Vec::new();
        let base_ciphertext = encryption_oracle(vec![].as_ref()).unwrap();
        // TODO: This shouldn't be necessary, probably an off-by-N error further down
        let mut should_continue: bool = true;
        while message.len() < base_ciphertext.len() && should_continue {
            let block_start = message.len();
            let block_end = block_start + block_size;

            // within the block, work backwards, build a dictionary of possible bytes, and match the cipher block
            // against the dictionary
            for i in (0..block_size).rev() {
                let prefix: Vec<u8> = (0..i).map(|_| 65).collect();
                let known_prefix = [prefix.to_owned(), message.to_owned()].concat();
                let dict = dictionary(known_prefix.as_ref(), block_start, block_end);
                let block = encryption_oracle(prefix.as_ref()).unwrap()[block_start..block_end].to_vec();
                let byte = dict.get::<Vec<u8>>(block.as_ref()).copied();
                if byte.is_some() {
                    message.push(byte.unwrap());
                } else {
                    should_continue = false;
                    break;
                }
            }
        }

        let final_message = pkcs7::unpad(message.as_ref());
        assert_eq!(UNKNOWN_STRING.to_vec(), final_message);
    }
}
