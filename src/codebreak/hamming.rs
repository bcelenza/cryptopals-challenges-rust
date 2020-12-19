pub mod hamming {
    use std::error::Error;

    pub fn distance(input1: &[u8], input2: &[u8]) -> Result<usize, Box<dyn Error>> {
        let diff = input1.iter().zip(input2.iter()).map(|(x1, x2)| x1 ^ x2).collect();
        weight(diff)
    }

    pub fn weight(input: Vec<u8>) -> Result<usize, Box<dyn Error>> {
        // Assuming the CPU supports SSE4.2 and taking the easy way out
        Ok(input.iter().fold(0, |x1, x2| x1 + x2.count_ones() as usize))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_distance() {
            let input1 = String::from("this is a test");
            let input2 = String::from("wokka wokka!!!");
            assert_eq!(37, distance(&input1.as_bytes(), &input2.as_bytes()).unwrap());
        }
    }
}