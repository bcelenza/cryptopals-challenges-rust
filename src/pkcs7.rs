pub mod pkcs7 {
    pub fn pad(input: &Vec<u8>, key_size: &usize) -> Vec<u8> {
        let pad_num: usize;
        if input.len() == *key_size {
            pad_num = *key_size;
        } else if input.len() < *key_size {
            pad_num = *key_size - input.len();
        } else {
            pad_num = *key_size - input.len() % *key_size;
        }
        [input.clone(), vec![pad_num as u8; pad_num]].concat()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_pad_equal_size() {
            let input = vec![0; 16];
            let key_size = 16; 
            let result = pad(&input, &key_size);
            let expected = [input, vec![16; 16]].concat();
            assert_eq!(expected, result);
        }

        #[test]
        fn test_pad_key_larger() {
           let input = vec![0; 16];
           let key_size = 20;
           let result = pad(&input, &key_size);
           let expected = [input, vec![4; 4]].concat();
           assert_eq!(expected, result);
        }

        #[test]
        fn test_pad_input_larger() {
           let input = vec![0; 32];
           let key_size = 20;
           let result = pad(&input, &key_size);
           let expected = [input, vec![8; 8]].concat();
           assert_eq!(expected, result);
        }
    }
}
