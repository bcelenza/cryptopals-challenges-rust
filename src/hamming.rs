pub mod hamming {
    use std::error::Error;

    pub fn distance(input1: &str, input2: &str) -> Result<usize, Box<dyn Error>> {
        let b1 = input1.as_bytes();
        let b2 = input2.as_bytes();
        let diff = b1.iter().zip(b2.iter()).map(|(x1, x2)| x1 ^ x2).collect();
        let weight = weight(diff)?;
        Ok(weight)
    }

    pub fn weight(input: Vec<u8>) -> Result<usize, Box<dyn Error>> {
        // naive approach for now
        Ok(input.iter().fold(0, |x1, x2| x1 + x2.count_ones() as usize))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_distance() {
            let input1 = String::from("this is a test");
            let input2 = String::from("wokka wokka!!!");
            assert_eq!(37, distance(&input1, &input2).unwrap());
        }
    }
}