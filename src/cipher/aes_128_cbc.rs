pub mod aes_128_cbc {
    use crate::pkcs7::*;
    use std::error::Error;
    use openssl::symm::{Cipher, Crypter, Mode};

    pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None)?;
        decrypter.pad(false);

        let blocks = ciphertext.chunks(key.len());
        let mut last_block: Vec<u8> = iv.to_owned();
        let mut result: Vec<u8> = Vec::with_capacity(ciphertext.len());
        for block in blocks {
            // to avoid panics by the decrypter:
            // give extra capacity to the decrypted block
            let mut decrypted = vec![0; 2 * block.len()];
            // expect an error here
            decrypter.update(block, &mut decrypted).expect("Decrypt error.");
            // expect an error here
            decrypter.finalize(&mut decrypted).expect("Decrypt error.");
            // truncate the block back down to the appropriate size
            decrypted.truncate(block.len());

            // XOR against the IV or last ciphertext block
            let mut block_result = decrypted.iter()
                .zip(last_block)
                .map(|(c, v)| c ^ v)
                .collect();
            result.append(&mut block_result);
            last_block = block.to_vec();
        }
        Ok(pkcs7::unpad(&result))
    }
}