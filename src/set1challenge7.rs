pub mod set1challenge7 {
    use std::error::Error;

    pub fn solve(input: &str) -> Result<String, Box<dyn Error>> {
        // convert string to individual hex bytes
        let ciphertext = base64::decode(input.replace("\n", ""))?;
        let key = "YELLOW SUBMARINE";
        let cipher = openssl::symm::Cipher::aes_128_ecb();
        let decrypted = openssl::symm::decrypt(cipher, key.as_bytes(), None, &ciphertext)?;
        let result = String::from_utf8(decrypted)?;
        Ok(result)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_s1c7() {
            let input = fs::read_to_string("challenge7.txt").unwrap();
            let start = time::Instant::now();
            let result = solve(&input).unwrap();
            let solve_time = start.elapsed().whole_microseconds();
            assert_eq!(fs::read_to_string("challenge6-decrypted.txt").unwrap(), result);
            println!("Set 1 Challenge 7 took {}us to solve.", solve_time);
        }
    }
}
