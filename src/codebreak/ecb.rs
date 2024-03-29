use std::collections::HashMap;
use std::error::Error;

pub fn is_ecb(ciphertext: &[u8], key_size_bytes: &usize) -> bool {
    // search for repeated blocks in the ciphertext, summing how many
    // repeated blocks were found at the end
    let mut block_frequencies: HashMap<&[u8], usize> = HashMap::new();
    ciphertext.chunks(*key_size_bytes).for_each(|seq| {
        let frequency = block_frequencies.entry(seq).or_insert(0);
        *frequency += 1;
    });
    let repeated_sum = block_frequencies.values()
        .fold(0, |acc, val| if *val > 1 { acc + val } else { acc });
    repeated_sum > 1
}

pub fn determine_block_size(max_len: usize, encrypt_routine: fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>>) -> usize {
    let mut block_size: usize = 0;
    let mut last_ciphertext_size: usize = 0;
    for s in 1..max_len+1 {
        let input: Vec<u8> = (0..s).map(|_| 65).collect();
        let ciphertext = encrypt_routine(input.as_ref()).unwrap();
        if last_ciphertext_size > 0 && ciphertext.len() > last_ciphertext_size {
            block_size = ciphertext.len() - last_ciphertext_size;
            break;
        }
        last_ciphertext_size = ciphertext.len();
    }
    block_size
}

pub fn build_byte_dictionary(prefix: &[u8], block_start: usize, block_end: usize, encrypt_routine: fn(&[u8]) -> Result<Vec<u8>, Box<dyn Error>>) -> HashMap<Vec<u8>, u8> {
    let mut dictionary: HashMap<Vec<u8>, u8> = HashMap::new();
    for byte in 0..u8::MAX {
        let input = [prefix.to_owned(), vec![byte]].concat();
        let block = encrypt_routine(input.as_ref()).unwrap()[block_start..block_end].to_vec();
        dictionary.insert(block, byte);
    }
    dictionary
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use crate::cipher::aes_128_cbc;
    use crate::cipher::aes_128_ecb;

    #[test]
    fn test_is_ecb_true() {
        let plaintext: Vec<u8> = (0..128).map(|_| 220u8 ).collect();
        let mut rng = rand::thread_rng();
        let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let ciphertext = aes_128_ecb::encrypt(&plaintext, &key, None).unwrap();
        assert_eq!(true, is_ecb(&ciphertext, &16));
    }

    #[test]
    fn test_is_ecb_false() {
        let plaintext: Vec<u8> = (0..128).map(|_| 220u8 ).collect();
        let mut rng = rand::thread_rng();
        let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let ciphertext = aes_128_cbc::encrypt(&plaintext, &key, &iv).unwrap();
        assert_eq!(false, is_ecb(&ciphertext, &16));
    }
}