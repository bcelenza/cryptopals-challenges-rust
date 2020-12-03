pub mod set1challenge6 {
    use std::error::Error;
    use std::slice::Chunks;
    use crate::hamming::*;
    use crate::set1challenge3::*;
    use crate::set1challenge5::*;

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
        println!("Key size is probably {} (normalized hamming distance: {:.3})", probable_key_size.key_size, probable_key_size.normalized_distance);

        // split the cipher text into blocks of key_size size
        let blocks = ciphertext.chunks(probable_key_size.key_size);

        let mut key: Vec<u8> = Vec::new();
        for key_byte in 0..probable_key_size.key_size {
            key.push(find_key_byte(&blocks, key_byte));
        }

        let value = String::from_utf8(set1challenge5::decrypt_repeating_xor(&ciphertext, &key))?;

        Ok(Answer {
            value: value,
            key: key,
        })
    }

    struct KeySize {
        pub key_size: usize,
        pub normalized_distance: f32,
    }

    fn find_probable_key_size(ciphertext: &Vec<u8>) -> Result<KeySize, Box<dyn Error>> {
        let mut probable_key_size: Option<KeySize> = None;

        for key_size in 2..40 {
            let mut distance: usize = 0;
            let blocks = ciphertext.len() / key_size;

            for block in 1..blocks {
                let slice1 = &ciphertext[(block - 1) * key_size .. block * key_size];
                let slice2 = &ciphertext[key_size * block .. (block + 1) * key_size];
                distance += hamming::distance_slice(slice1, slice2)?;
            }
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

    fn find_key_byte(blocks: &Chunks<u8>, offset: usize) -> u8 {
        let mut best_score = 0;
        let mut probable_key: u8 = 0;

        // for test_key in 0..u8::MAX {
            // let ciphertext_block_bytes = blocks.partition(f: F).clone().map(|b| b[offset] ^ test_key).collect();
            // let score = set1challenge3::score(&ciphertext_block_bytes);
            // if score >= best_score {
                // best_score = score;
                // probable_key = test_key;
            // }
        // }

        println!("Key byte {} is {} (score: {})", offset, probable_key, best_score);

        probable_key
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_s1c6() {
            let input = fs::read_to_string("challenge6.txt").unwrap();
            let start = time::Instant::now();
            let result = solve(&input).unwrap();
            let solve_time = start.elapsed().whole_microseconds();
            println!("Set 1 Challenge 6 result: {}", result.value);
            println!("Set 1 Challenge 6 took {}us to solve.", solve_time);
        }
    }
}
