use std::error::Error;
use crate::set1::challenge3::*;

pub fn solve(input: &str) -> Result<Answer, Box<dyn Error>> {
    let lines = input.lines();
    let mut answer: Option<Answer> = None;
    for line in lines {
        let a = crate::set1::challenge3::solve(&line).unwrap();
        if answer.as_ref().is_none() || answer.as_ref().unwrap().score <= a.score {
            answer.replace(a);
        }
    }

    Ok(answer.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("data/set1/challenge4.txt").unwrap();
        let result = solve(&input);
        let value = String::from_utf8(result.unwrap().value).unwrap();
        assert_eq!("Now that the party is jumping\n", value);
    }
}
