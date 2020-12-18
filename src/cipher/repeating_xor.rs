pub mod repeating_xor {
    pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
        ciphertext.iter()
            .zip(key.iter().cycle())
            .map(|(x1, x2)| x1 ^ x2)
            .collect()
    }
}