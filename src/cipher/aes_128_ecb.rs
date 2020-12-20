pub mod aes_128_ecb {
    use std::error::Error;

    pub fn encrypt(input: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, Box<dyn Error>> {
        let cipher = openssl::symm::Cipher::aes_128_ecb();
        let result = openssl::symm::encrypt(cipher, key, iv, input)?;
        Ok(result)
    }
    
    pub fn decrypt(input: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, Box<dyn Error>> {
        let cipher = openssl::symm::Cipher::aes_128_ecb();
        let result = openssl::symm::decrypt(cipher, key, iv, input)?;
        Ok(result)
    }
}