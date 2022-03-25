use std::error::Error;
use crate::cipher::aes_128_ecb;

pub fn solve(input: &str) -> Result<String, Box<dyn Error>> {
    // convert string to individual hex bytes
    let ciphertext = base64::decode(input.replace("\n", ""))?;
    let key = b"YELLOW SUBMARINE";
    let decrypted = aes_128_ecb::decrypt(&ciphertext, key, None)?;
    let result = String::from_utf8(decrypted)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("data/set1/challenge7.txt").unwrap();
        let result = solve(&input).unwrap();
        assert_eq!(fs::read_to_string("data/set1/challenge6-decrypted.txt").unwrap(), result);
    }
}
