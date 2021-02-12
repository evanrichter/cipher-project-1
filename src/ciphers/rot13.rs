use crate::utils::{CharToNum, NumToChar};
use crate::Cipher;

pub struct Rot13;

impl Cipher for Rot13 {
    fn encrypt<'d>(&self, plaintext: &str) -> String {
        plaintext
            .chars()
            .map(|c| (c.to_num() + 13) % 27)
            .map(|x| x.to_char())
            .collect()
    }

    fn decrypt<'d>(&self, ciphertext: &str) -> String {
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

    #[test]
    fn round_trip() {
        let rot13 = Rot13;

        // assert encryption works as expected
        let plaintext = "abcdefghijklmnopqrstuvwxyz ";
        let ciphertext = rot13.encrypt(&plaintext);
        assert_eq!(ciphertext, "nopqrstuvwxyz abcdefghijklm");

        // assert decryption produces the same plaintext
        let decrypted = rot13.decrypt(&ciphertext);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn stresstest() {
        crate::ciphers::testing::stresstest(Rot13, 10000).unwrap();
    }
}
