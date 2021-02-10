use crate::utils::{CharToNum, NumToChar};
use crate::Cipher;
use crate::Dictionary;

pub struct Rot13;

impl Cipher for Rot13 {
    fn encrypt<'d>(&self, plaintext: &str, _dict: &'d Dictionary) -> String {
        plaintext
            .chars()
            .map(|c| (c.to_num() + 13) % 27)
            .map(|x| x.to_char())
            .collect()
    }

    fn decrypt<'d>(&self, ciphertext: &str, _dict: &'d Dictionary) -> String {
        ciphertext
            .chars()
            .map(|c| (c.to_num() + (27 - 13)) % 27)
            .map(|x| x.to_char())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen::Generator;

    #[test]
    fn round_trip() {
        let rot13 = Rot13;
        let dict = Dictionary::from_string("".to_string());

        // assert encryption works as expected
        let plaintext = "abcdefghijklmnopqrstuvwxyz ";
        let ciphertext = rot13.encrypt(&plaintext, &dict);
        assert_eq!(ciphertext, "nopqrstuvwxyz abcdefghijklm");

        // assert decryption produces the same plaintext
        let decrypted = rot13.decrypt(&ciphertext, &dict);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn stresstest() {
        crate::ciphers::testing::stresstest(Rot13, 10000).unwrap();
    }
}
