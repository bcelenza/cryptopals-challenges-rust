use std::error::Error;
use std::collections::HashMap;

pub fn solve(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded: Result<Vec<_>, _> = input.lines().map(|e| hex::decode(e)).collect();
    let lines = decoded?; 
    let key_size_bytes = 16;
    
    // search for repeated blocks in each line, summing how many
    // repeated blocks were found at the end
    let mut repeated_blocks: Vec<usize> = vec![0; lines.len()];
    for (line_num, line) in lines.iter().enumerate() {
        let mut block_frequencies: HashMap<&[u8], usize> = HashMap::new();
        line.chunks(key_size_bytes).for_each(|seq| {
            let frequency = block_frequencies.entry(seq).or_insert(0);
            *frequency += 1;
        });
        repeated_blocks[line_num] = block_frequencies.values()
            .fold(0, |acc, val| if *val > 1 { acc + val } else { acc });
    }

    // determine which line had the most repeated blocks
    let mut probable_line: usize = 0;
    let mut most_repeats: usize = 0;
    for (line, count) in repeated_blocks.iter().enumerate() {
        if count >= &most_repeats {
            most_repeats = *count;
            probable_line = line;
        }
    }

    Ok(lines[probable_line].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_s1c8() {
        let input = fs::read_to_string("data/set1/challenge8.txt").unwrap();
        let result = solve(&input).unwrap();
        // line 133 is expected
        let expected = String::from("d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a");
        assert_eq!(expected, hex::encode(result));
    }
}
