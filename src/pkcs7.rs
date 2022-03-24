pub mod pkcs7 {
    use std::fmt;
    use std::error::Error;

    type Result<T> = std::result::Result<T, Pkcs7UnpadError>;

    #[derive(Debug)]
    pub struct Pkcs7UnpadError;

    impl fmt::Display for Pkcs7UnpadError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid pkcs7 padding")
        }
    }

    impl Error for Pkcs7UnpadError {
        fn description(&self) -> &str {
            "PKCS7 unpadding error"
        }
    }

    pub fn pad(input: &[u8], key_size: usize) -> Vec<u8> {
        let pad_num: usize;
        pad_num = key_size - (input.len() % key_size);
        [input.to_vec(), vec![pad_num as u8; pad_num]].concat()
    }
    
    pub fn unpad(input: &[u8], key_size: usize) -> Result<Vec<u8>> {
        let pad_num = *input.last().unwrap() as usize;
        if pad_num == 0 || pad_num  > key_size {
            // invalid padding character
            return Err(Pkcs7UnpadError)
        }
        for i in input[input.len() - pad_num..input.len()].to_vec() {
            if i != pad_num as u8 {
                // inconsistent padding
                return Err(Pkcs7UnpadError)
            }
        }
        Ok(input[0..input.len() - pad_num].to_vec())
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
            let result = unpad(&input, 4).unwrap();
            assert_eq!(vec![0, 0, 0], result);
        }

        #[test]
        fn test_unpad_multi_byte() {
            let input = vec![0, 0, 0, 4, 4, 4, 4];
            let result = unpad(&input, 7).unwrap(); 
            assert_eq!(vec![0, 0, 0], result);
        }

        #[test]
        #[should_panic]
        fn test_unpad_invalid_input() {
            let input = vec![0, 0, 0, 0, 0, 4, 4, 4];
            unpad(&input, 8).unwrap();
        }
            
    }
}
