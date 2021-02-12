//! Implementations of various ciphers.

mod rot13;
pub use rot13::Rot13;

/// The Cipher trait describes what every cipher needs to be able to do.
pub trait Cipher {
    /// Encrypt the given plaintext and return a String
    fn encrypt<'d>(&self, plaintext: &str) -> String;
    /// Decrypt the given ciphertext and return a String
    fn decrypt<'d>(&self, ciphertext: &str) -> String;
}

#[cfg(test)]
pub mod testing {
    use super::*;
    use crate::dict::Dictionary;
    use crate::gen::Generator;

    pub fn stresstest<T: Cipher>(cipher: T, cycles: usize) -> anyhow::Result<()> {
        let words = std::fs::read_to_string("words/default.txt")?;
        let dict = Dictionary::from_string(words);
        let mut gen = Generator::with_dict(&dict);

        for _ in 0..cycles {
            // pick number of words to generate
            let num_words = usize::max(10, gen.rng.next() as usize % 150);

            // generate plaintext, ciphertext, and then decrypt
            let plaintext = gen.generate_words(num_words);
            println!("{}", plaintext);
            let ciphertext = cipher.encrypt(&plaintext);
            let decrypted = cipher.decrypt(&ciphertext);

            // plaintext must always differ from ciphertext
            assert_ne!(plaintext, ciphertext);

            // decrypted text must always match original plaintext
            assert_eq!(plaintext, decrypted);
        }

        Ok(())
    }
}
