use std::error::Error;
use crate::codebreak::hamming;
use crate::cipher::repeating_xor;
use crate::set1::challenge3::score;

#[derive(Clone)]
pub struct Answer {
    pub value: String,
    pub key: Vec<u8>,
}

pub fn solve(input: &str) -> Result<Answer, Box<dyn Error>> {
    // decode the input
    let ciphertext = base64::decode(input.replace("\n", ""))?;

    // find the probable key size for the ciphertext
    let probable_key_size = find_probable_key_size(&ciphertext)?;
    let key_size = probable_key_size.key_size;

    // split the cipher text into key_size blocks of every nth element
    let blocks = ciphertext.chunks(key_size);
    let mut transposed_blocks: Vec<Vec<u8>> = vec![Vec::with_capacity(blocks.len()); key_size];
    blocks.for_each(|b| b.iter().enumerate().for_each(|(i, x)| transposed_blocks[i].push(*x)));

    // cycle through every key byte to identify what the value of that byte should be
    let mut key: Vec<u8> = Vec::new();
    for key_byte in 0..key_size {
        key.push(find_key_byte(&transposed_blocks[key_byte]));
    }

    // decrypt this thing
    let value = String::from_utf8(repeating_xor::decrypt(&ciphertext, &key))?;

    Ok(Answer {
        value: value,
        key: key,
    })
}

struct KeySize {
    pub key_size: usize,
    pub normalized_distance: f32,
}

fn find_probable_key_size(ciphertext: &[u8]) -> Result<KeySize, Box<dyn Error>> {
    let mut probable_key_size: Option<KeySize> = None;

    // search for key size from 2-40 characters
    for key_size in 2..40 {
        let mut distance: usize = 0;
        let blocks = ciphertext.len() / key_size;

        // iterate through pairs of blocks and compare their distance
        for block in 1..blocks {
            let slice1 = &ciphertext[(block - 1) * key_size .. block * key_size];
            let slice2 = &ciphertext[key_size * block .. (block + 1) * key_size];
            distance += hamming::distance(slice1, slice2);
        }

        // normalize that distance by the key size and number of blocks
        let normalized_distance = distance as f32 / key_size as f32 / blocks as f32;

        if probable_key_size.as_ref().is_none() || probable_key_size.as_ref().unwrap().normalized_distance >= normalized_distance {
            probable_key_size.replace(KeySize {
                key_size: key_size,
                normalized_distance: normalized_distance,
            });
        }
    }

    Ok(probable_key_size.unwrap())
}

fn find_key_byte(bytes: &[u8]) -> u8 {
    let mut best_score = 0;
    let mut probable_key: u8 = 0;

    for test_key in 0..u8::MAX {
        let test_decoded: Vec<u8> = bytes.iter().map(|b| b ^ test_key).collect();
        let score = score(&test_decoded);
        if score >= best_score {
            best_score = score;
            probable_key = test_key;
        }
    }

    probable_key
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_s1c6() {
        let input = fs::read_to_string("data/set1/challenge6.txt").unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(fs::read_to_string("data/set1/challenge6-decrypted.txt").unwrap(), result.value);
    }
}
