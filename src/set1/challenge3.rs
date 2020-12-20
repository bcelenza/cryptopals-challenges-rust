use std::error::Error;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Answer {
    pub value: Vec<u8>,
    pub key: u8,
    pub score: u32,
}
    
pub fn solve(input: &str) -> Result<Answer, Box<dyn Error>> {
    // decode hex input
    let bytes = hex::decode(input)?;

    // initialize key to zero
    let mut key: u8 = 0;

    // iterate over possible keys from 0 to 8-bit uint max
    let mut answer: Option<Answer> = None;
    while key < u8::MAX {
        // XOR the input against the target
        let value: Vec<u8> = bytes.iter().map(|x| x ^ key).collect();

        // determine the score for this result
        let score = score(&value);

        if answer.as_ref().is_none() || answer.as_ref().unwrap().score <= score {
            answer.replace(Answer {
                value: value,
                key: key,
                score: score
            });
        }

        key += 1;
    }

    Ok(answer.unwrap())
}

pub fn score(input: &[u8]) -> u32 {
    // using the top 10 english characters, develop a score weighted by their frequency
    // other characters will count as zero
    let char_weights: HashMap<u8, u8> = [
        (b' ', 15),
        (b'e', 11),
        (b'E', 11),
        (b'a', 8),
        (b'A', 8),
        (b'r', 7),
        (b'R', 7),
        (b'i', 7),
        (b'I', 7),
        (b'o', 7),
        (b'O', 7),
        (b't', 6),
        (b'T', 6),
        (b'n', 6),
        (b'N', 6),
        (b's', 5),
        (b'S', 5),
        (b'l', 5),
        (b'L', 5),
        (b'c', 4),
        (b'C', 4),
    ].iter().cloned().collect();

    // build the score for the input
    let score: u32 = input.iter().map(|x| {
        match char_weights.get(x) {
            Some(v) => u32::from(*v),
            None => 0
        }
    }).sum();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s1c3() {
        let input = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let result = solve(&input).unwrap();
        assert_eq!("Cooking MC's like a pound of bacon", String::from_utf8(result.value).unwrap());
    }
}
