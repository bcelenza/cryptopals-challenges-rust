
use std::error::Error;
use crate::pkcs7::*;

pub fn solve(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes = input.as_bytes().to_vec();
    let key_size: usize = 20;
    Ok(pkcs7::pad(&bytes, &key_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s2c9() {
        let input = String::from("YELLOW SUBMARINE");
        let start = time::Instant::now();
        let result = solve(&input).unwrap();
        let expected = [input.as_bytes().to_vec(), vec![4; 4]].concat();
        assert_eq!(expected, result);
        let solve_time = start.elapsed().whole_microseconds();
        println!("Set 2 Challenge 9 took {}us to solve.", solve_time);
    }
}
