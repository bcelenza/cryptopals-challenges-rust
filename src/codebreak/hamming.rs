pub fn distance(input1: &[u8], input2: &[u8]) -> usize {
    let diff = input1.iter().zip(input2.iter()).map(|(x1, x2)| x1 ^ x2).collect();
    weight(diff)
}

pub fn weight(input: Vec<u8>) -> usize {
    // Assuming the CPU supports SSE4.2 and taking the easy way out
    input.iter().fold(0, |x1, x2| x1 + x2.count_ones() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let input1 = b"this is a test";
        let input2 = b"wokka wokka!!!";
        assert_eq!(37, distance(input1, input2));
    }
}