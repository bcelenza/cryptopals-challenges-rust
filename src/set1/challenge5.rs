use std::error::Error;
use crate::cipher::repeating_xor;

pub fn solve(input: &str) -> Result<String, Box<dyn Error>> {
    let key = b"ICE";
    let result = repeating_xor::decrypt(input.as_bytes(), key);
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
        let result = solve(&input).unwrap();
        let expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(expected, result);
    }
}
