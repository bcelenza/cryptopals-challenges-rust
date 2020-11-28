pub mod set1challenge5 {
    use std::error::Error;

    pub fn solve(input: &str) -> Result<String, Box<dyn Error>> {
        let key = "ICE".as_bytes();
        let result: Vec<u8> = input.as_bytes().iter()
            .zip(key.iter().cycle())
            .map(|(x1, x2)| x1 ^ x2)
            .collect();
        Ok(hex::encode(result))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let input = String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
            let start = time::Instant::now();
            let result = solve(&input).unwrap();
            let solve_time = start.elapsed().whole_microseconds();
            let expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
            assert_eq!(expected, result);
            println!("Set 1 Challenge 5 took {}us to solve.", solve_time);
        }
    }
}
