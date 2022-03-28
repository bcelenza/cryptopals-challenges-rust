use crate::cipher::aes_128_ecb;
use lazy_static::lazy_static;
use rand::Rng;
use std::error::Error;

lazy_static! {
    // create a random static key
    static ref KEY: Vec<u8> = {
        let mut rng = rand::thread_rng();
        (0..16).map(|_| rng.gen()).collect()
    };
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ProfileInfo {
    email: String,
    uid: u32,
    role: String
}

pub fn encode_profile(profile: &ProfileInfo) -> String {
    format!("email={}&uid={}&role={}", profile.email, profile.uid, profile.role)
}

pub fn profile_for(email: String) -> Result<Vec<u8>, Box<dyn Error>> {
    // drop special characters
    let sanitized_email = email.replace("&", "").replace("=", "");
    let profile = ProfileInfo{
        email: sanitized_email,
        uid: 10,
        role: String::from("user")
    };
    let encoded = encode_profile(&profile).as_bytes().to_vec();
    
    // encrypt
    let encrypted = aes_128_ecb::encrypt(encoded.as_ref(), KEY.as_ref(), None)?;
    Ok(encrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_profile() {
        let profile = ProfileInfo{
            email: String::from("leo@beo.net"),
            uid: 25,
            role: String::from("dog")
        };
        let result = encode_profile(&profile);
        assert_eq!("email=leo@beo.net&uid=25&role=dog", result);
    }

    #[test]
    fn test_profile_for() {
        let result = profile_for(String::from("leo@beo.net")).unwrap();
        let decrypted = aes_128_ecb::decrypt(&result, KEY.as_ref(), None).unwrap();
        let expected = String::from("email=leo@beo.net&uid=10&role=user");
        assert_eq!(expected, String::from_utf8(decrypted).unwrap());
    }
    
    #[test]
    fn test_profile_for_dropped_chars() {
        let mut result = profile_for(String::from("leo@beo.net=")).unwrap();
        let mut decrypted = aes_128_ecb::decrypt(&result, KEY.as_ref(), None).unwrap();
        let expected = String::from("email=leo@beo.net&uid=10&role=user");
        assert_eq!(expected, String::from_utf8(decrypted).unwrap());

        result = profile_for(String::from("leo@beo.net&")).unwrap();
        decrypted = aes_128_ecb::decrypt(&result, KEY.as_ref(), None).unwrap();
        assert_eq!(expected, String::from_utf8(decrypted).unwrap());
    }

    #[test]
    fn test_solution() {
        let block_size = 16;
        let prefix_size = "email=".len();
        let suffix_size = "&uid=10&role=".len();

        // craft the start of an email that occupies the first block
        let mut email = "A".repeat(block_size - prefix_size);

        // fill the second block with the value `admin` plus valid PKCS7 padding
        let role = "admin";
        email.push_str(role);
        let pkcs7_padding = "\u{000B}".repeat(11);
        email.push_str(pkcs7_padding.as_str());

        // pad the start of the third block to align role= to the end of the block
        let block3_padding = "A".repeat(block_size - suffix_size);
        email.push_str(block3_padding.as_str());

        // get the encrypted profile
        let encrypted = profile_for(email).unwrap();

        // re-create the encrypted text, cutting block 2 and appending it to the end
        let block1 = encrypted[0..16].to_vec();
        let block2 = encrypted[16..32].to_vec();
        let block3 = encrypted[32..48].to_vec();
        let forged = [block1, block3, block2].concat();

        let expected = String::from("email=AAAAAAAAAAAAA&uid=10&role=admin");
        let decrypted = aes_128_ecb::decrypt(forged.as_ref(), KEY.as_ref(), None).unwrap();
        assert_eq!(expected, String::from_utf8(decrypted).unwrap());
    }
}
