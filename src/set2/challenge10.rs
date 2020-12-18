
use std::error::Error;
use crate::cipher::aes_128_cbc::*;

pub fn solve(ciphertext: &str, key: &str, iv: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = base64::decode(ciphertext.replace("\n", ""))?;
    Ok(aes_128_cbc::decrypt(decoded.as_ref(), key.as_bytes().to_vec().as_ref(), iv)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_s2c10() {
        let key = String::from("YELLOW SUBMARINE");
        let iv = vec![0; key.len()];
        let input = fs::read_to_string("data/set2/challenge10.txt").unwrap();
        let expected = fs::read_to_string("data/set2/challenge10-decrypted.txt").unwrap();
        let start = time::Instant::now();
        let result = solve(&input, &key, &iv).unwrap();
        let solve_time = start.elapsed().whole_microseconds();
        assert_eq!(expected, String::from_utf8(result).unwrap());
        println!("Set 2 Challenge 10 took {}us to solve.", solve_time);
    }
}
