//! Implementations of various ciphers.

mod encryptor;
mod rot13;
pub mod schedulers;

pub use encryptor::Encryptor;
pub use rot13::Rot13;
pub use schedulers::KeySchedule;

/// The Cipher trait describes what every cipher needs to be able to do.
pub trait Cipher {
    /// Encrypt into an already allocated String, appending ciphertext
    fn encrypt_into(&self, plaintext: &str, ciphertext: &mut String);

    /// Decrypt the given ciphertext and return a String.
    fn decrypt_into(&self, ciphertext: &str, plaintext: &mut String);

    /// Decrypt the given ciphertext and return a String.
    fn decrypt(&self, ciphertext: &str) -> String {
        let mut plaintext = String::with_capacity(ciphertext.len());
        self.decrypt_into(ciphertext, &mut plaintext);
        plaintext
    }

    /// Encrypt the given plaintext and return a String
    fn encrypt(&self, plaintext: &str) -> String {
        let mut ciphertext = String::with_capacity(plaintext.len());
        self.encrypt_into(plaintext, &mut ciphertext);
        ciphertext
    }
}

#[cfg(test)]
pub mod testing {
    use super::*;
    use crate::dict::Dictionary;
    use crate::gen::Generator;
    use crate::rng::{FromRng, Rng};

    use std::fmt::Debug;

    fn test_one<T: Cipher + Debug>(cipher: &T, gen: &mut Generator) {
        // pick number of words to generate
        let num_words = usize::max(10, gen.rng.next() as usize % 150);

        // generate plaintext, ciphertext, and then decrypt
        let plaintext = gen.generate_words(num_words);
        let ciphertext = cipher.encrypt(&plaintext);
        let decrypted = cipher.decrypt(&ciphertext);

        // plaintext must always differ from ciphertext
        if plaintext == ciphertext {
            dbg!(&cipher);
        }
        assert_ne!(plaintext, ciphertext);

        // decrypted text must always match original plaintext
        assert_eq!(plaintext, decrypted);
    }

    pub fn stresstest<T: Cipher + Debug>(cipher: T, cycles: usize) -> anyhow::Result<()> {
        let mut words = std::fs::read_to_string("words/default.txt")?;
        let dict = Dictionary::from_string(&mut words);
        let mut gen = Generator::with_dict(&dict);

        for _ in 0..cycles {
            test_one(&cipher, &mut gen);
        }

        Ok(())
    }

    pub fn randomized_stresstest<T: Cipher + FromRng + Debug>(cycles: usize) -> anyhow::Result<()> {
        let mut rng = Rng::default();

        let mut words = std::fs::read_to_string("words/default.txt")?;
        let dict = Dictionary::from_string(&mut words);
        let mut gen = Generator::with_dict(&dict);

        for _ in 0..cycles {
            let cipher = T::from_rng(&mut rng);
            test_one(&cipher, &mut gen);
        }

        Ok(())
    }

    #[test]
    fn aab_stress() {
        randomized_stresstest::<Encryptor<schedulers::Aab>>(10000).unwrap();
    }
}
