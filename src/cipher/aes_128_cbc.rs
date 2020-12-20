pub mod aes_128_cbc {
    use crate::pkcs7::*;
    use std::error::Error;
    use openssl::symm::{Cipher, Crypter, Mode};

    pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut encrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, Some(iv))?;
        encrypter.pad(false);

        let padded = pkcs7::pad(data, key.len());

        let blocks = padded.chunks(key.len());
        let mut last_block = iv.to_vec();
        let mut result: Vec<u8> = Vec::with_capacity(padded.len());
        for block in blocks {
            // XOR with the IV or last block
            let coded: Vec<u8> = block.iter()
                .zip(last_block)
                .map(|(c, v)| c ^ v)
                .collect();

            // to avoid panics by the encrypter:
            // give extra capacity to the decrypted block
            let mut encrypted = vec![0u8; 2 * block.len()];
            
            // expect errors on the update and finalize operations
            encrypter.update(&coded, &mut encrypted).expect("Encrypt error");
            encrypter.finalize(&mut encrypted).expect("Encrypt error");

            // remove the extra capacity from the block
            encrypted.truncate(block.len());

            last_block = encrypted.clone();
            result.append(&mut encrypted);
        }

        Ok(result)
    }

    pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, Some(iv))?;
        decrypter.pad(false);

        let blocks = ciphertext.chunks(key.len());
        let mut last_block = iv.to_vec();
        let mut result: Vec<u8> = Vec::with_capacity(ciphertext.len());
        for block in blocks {
            // to avoid panics by the decrypter:
            // give extra capacity to the decrypted block
            let mut decrypted = vec![0; 2 * block.len()];
            // expect errors on the update and finalize operations
            decrypter.update(block, &mut decrypted).expect("Decrypt error.");
            decrypter.finalize(&mut decrypted).expect("Decrypt error.");

            // remove the extra capacity from the block
            decrypted.truncate(block.len());

            // XOR with the IV or last ciphertext block
            decrypted = decrypted.iter()
                .zip(last_block)
                .map(|(c, v)| c ^ v)
                .collect();

            result.append(&mut decrypted);
            last_block = block.to_vec();
        }
        Ok(pkcs7::unpad(&result))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand::Rng;

        #[test]
        fn test_encrypt() {
            let mut rng = rand::thread_rng();
            let data = b"You know they got me trapped in this prison of seclusion\n 
                         Happiness, living on the streets is a delusion";
            let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
            let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();

            let ciphertext = encrypt(data, &key, &iv).unwrap();
            let expected_ciphertext = openssl::symm::encrypt(Cipher::aes_128_cbc(), &key, Some(&iv), data).unwrap();
            assert_eq!(expected_ciphertext, ciphertext);
        }

        #[test]
        fn test_decrypt() {
            let mut rng = rand::thread_rng();
            let data = b"You know they got me trapped in this prison of seclusion\n 
                         Happiness, living on the streets is a delusion";
            let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
            let iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
            let ciphertext = openssl::symm::encrypt(Cipher::aes_128_cbc(), &key, Some(&iv), data).unwrap();

            let decrypted = decrypt(&ciphertext, &key, &iv).unwrap();
            assert_eq!(data.to_vec(), decrypted);

        }
    }
}