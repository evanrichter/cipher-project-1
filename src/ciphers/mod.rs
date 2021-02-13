//! Implementations of various ciphers.

mod encryptor;
mod rot13;

pub use encryptor::Encryptor;
pub use rot13::Rot13;

/// The Cipher trait describes what every cipher needs to be able to do.
pub trait Cipher {
    /// Encrypt the given plaintext and return a String
    fn encrypt(&self, plaintext: &str) -> String;
    /// Decrypt the given ciphertext and return a String.
    fn decrypt(&self, ciphertext: &str) -> String;
}

/// Returns the index of the key to use when shifting plaintext into ciphertext.
///
/// Arguments are: `index: usize`, `key_length: usize`, `plaintext_length: usize`.
///
/// In the project description, this process is described as: each ciphertext symbol `c[i]` is the
/// shift of the plaintext symbol `m[i]` by a number of position equal to one of the key symbols,
/// which symbol being chosen according to an _undisclosed, deterministic, and not key-based_,
/// scheduling algorithm that is a function of `i`, `t` and `L`, where:
///   * `i` is the index being output to ciphertext
///   * `t` is the key length
///   * `L` is the length of the plaintext
pub type KeySchedule = &'static dyn Fn(usize, usize, usize) -> usize;

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
