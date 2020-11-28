pub mod set1challenge1 {
    use std::error::Error;

    pub fn solve(input: &str) -> Result<String, Box<dyn Error>> {
        // convert string to individual hex bytes
        let bytes = hex::decode(input)?;
        // return base64 encoded value
        Ok(base64::encode(bytes))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_s1c1() {
            let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
            let start = time::Instant::now();
            let result = solve(&input).unwrap();
            let solve_time = start.elapsed().whole_microseconds();

            assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", result);
            println!("Set 1 Challenge 1 took {}us to solve.", solve_time);
        }
    }
}
