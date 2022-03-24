
pub mod pkcs7 {
    pub fn pad(input: &[u8], key_size: usize) -> Vec<u8> {
        let pad_num: usize;
        pad_num = key_size - (input.len() % key_size);
        [input.to_vec(), vec![pad_num as u8; pad_num]].concat()
    }
    
    pub fn unpad(input: &[u8]) -> Vec<u8> {
        let pad_num = input.last().unwrap();
        input[0..input.len() - *pad_num as usize].to_vec()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pad_equal_size() {
            let input = vec![0; 16];
            let key_size = 16; 
            let result = pad(&input, key_size);
            let expected = [input, vec![16; 16]].concat();
            assert_eq!(expected, result);
        }

        #[test]
        fn test_pad_key_larger() {
           let input = vec![0; 16];
           let key_size = 20;
           let result = pad(&input, key_size);
           let expected = [input, vec![4; 4]].concat();
           assert_eq!(expected, result);
        }

        #[test]
        fn test_pad_input_larger() {
           let input = vec![0; 32];
           let key_size = 20;
           let result = pad(&input, key_size);
           let expected = [input, vec![8; 8]].concat();
           assert_eq!(expected, result);
        }

        #[test]
        fn test_unpad_single_byte() {
            let input = vec![0, 0, 0, 1];
            let result = unpad(&input);
            assert_eq!(vec![0, 0, 0], result);
        }

        #[test]
        fn test_unpad_multi_byte() {
            let input = vec![0, 0, 0, 4, 4, 4, 4];
            let result = unpad(&input);
            assert_eq!(vec![0, 0, 0], result);
        }
    }
}
