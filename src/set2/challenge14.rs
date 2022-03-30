use crate::cipher::aes_128_ecb;
use lazy_static::lazy_static;
use rand::Rng;
use std::error::Error;

lazy_static! {
    // create a random static key
    static ref KEY: Vec<u8> = {
        let mut rng = rand::thread_rng();
        (0..16).map(|_| rng.gen()).collect()
    };

    // create a random static prefix and target bytes
    static ref PREFIX: Vec<u8> = {
        let mut rng = rand::thread_rng();
        (0..26).map(|_| rng.gen()).collect()
    };
    // create the decoded unknown string for comparison
    static ref UNKNOWN_STRING: Vec<u8> = base64::decode(String::from("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").as_bytes().to_vec()).unwrap();
}

pub fn encryption_oracle(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // append static text
    let data = [PREFIX.to_vec(), input.to_vec(), UNKNOWN_STRING.to_vec()].concat();

    // encrypt
    let encrypted = aes_128_ecb::encrypt(&data, KEY.as_ref(), None)?;
    Ok(encrypted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codebreak::ecb;
    use crate::pkcs7::pkcs7;

    #[test]
    fn test_solution() {
        // find block size based on when the ciphertext length changes
        let block_size = ecb::determine_block_size(64, encryption_oracle);
        assert_eq!(block_size, KEY.len());

        // insert a block to determine where in the ciphertext the input (and thus the target text) starts
        let base_ciphertext = encryption_oracle(vec![].as_ref()).unwrap();
        let offset_ciphertext = encryption_oracle(vec![65; block_size].as_ref()).unwrap();
        let mut changed_block_idx: Option<usize> = None;
        for (i, x) in base_ciphertext.iter().enumerate() {
            if offset_ciphertext[i] != *x {
                changed_block_idx = Some(i / block_size);
                break;
            }
        }
        assert!(changed_block_idx.is_some());
        let target_block_index = changed_block_idx.unwrap();
        assert!(PREFIX.len() <= (target_block_index + 1) * block_size);
        // ensure the base and offset ranges are equivalent
        let base_range_start = (target_block_index + 1) * block_size;
        let offset_range_start = (target_block_index + 2) * block_size;
        assert_eq!(base_ciphertext[base_range_start..base_range_start + block_size], offset_ciphertext[offset_range_start..offset_range_start + block_size]);

        // decrypt one block, one byte at a time
        let mut message: Vec<u8> = Vec::new();
        let target_range_start = target_block_index * block_size;
        let base_ciphertext = encryption_oracle(vec![].as_ref()).unwrap()[target_range_start..].to_vec();
        // TODO: This shouldn't be necessary, probably an off-by-N error further down
        let mut should_continue: bool = true;
        while message.len() < base_ciphertext.len() && should_continue {
            let block_start = target_range_start + message.len();
            let block_end = block_start + block_size;

            // within the block, work backwards, build a dictionary of possible bytes, and match the cipher block
            // against the dictionary
            for i in (0..block_size).rev() {
                let prefix: Vec<u8> = (0..i).map(|_| 65).collect();
                let known_prefix = [prefix.to_owned(), message.to_owned()].concat();
                let dict = ecb::build_byte_dictionary(known_prefix.as_ref(), block_start, block_end, encryption_oracle);
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

        println!("Message: {:?}", message.to_owned());

        assert_eq!(UNKNOWN_STRING.to_vec(), pkcs7::unpad(message.as_ref(), block_size).unwrap());
    }
}
