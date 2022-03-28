use std::error::Error;
use crate::cipher::aes_128_cbc;

pub fn solve(ciphertext: &str, key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = base64::decode(ciphertext.replace("\n", ""))?;
    Ok(aes_128_cbc::decrypt(decoded.as_ref(), key, iv)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_solution() {
        let key = b"YELLOW SUBMARINE";
        let iv = vec![0; key.len()];
        let input = fs::read_to_string("data/set2/challenge10.txt").unwrap();
        let expected = fs::read_to_string("data/set2/challenge10-decrypted.txt").unwrap();
        let result = solve(&input, key, &iv).unwrap();
        assert_eq!(expected, String::from_utf8(result).unwrap());
    }
}
