use std::error::Error;

pub fn solve(input: &str) -> Result<String, Box<dyn Error>> {
    // decode hex input
    let bytes = hex::decode(input)?;
    // create hex for key to XOR against
    let key = hex::decode(String::from("686974207468652062756c6c277320657965"))?;
    // XOR each byte of the input against the same byte offset in the target
    let result: Vec<u8> = bytes.iter().zip(key.iter()).map(|(&x1, &x2)| x1 ^ x2).collect();
    // re-encode the result
    Ok(hex::encode(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s1c2() {
        let input = String::from("1c0111001f010100061a024b53535009181c");
        let result = solve(&input).unwrap();
        assert_eq!("746865206b696420646f6e277420706c6179", result);
    }
}
