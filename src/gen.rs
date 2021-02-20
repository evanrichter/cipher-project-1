//! Module for [`Generator`].

use crate::dict::Dictionary;
use crate::rng::Rng;

/// A deterministic plaintext generator. The purpose is to be able to quickly generate known
/// plaintexts so that we can encipher them, and then attempt to crack the ciphertext. Since we
/// generated the plaintext ourself, we can simply compare our cracking results to verify.
#[derive(Clone, Debug)]
pub struct Generator<'d> {
    dictionary: &'d Dictionary,
    pub rng: Rng,
}

impl<'d> Generator<'d> {
    /// Instantiate a generator that generates messages using the given [`Dictionary`] as a
    /// wordbank.
    pub fn with_dict(dictionary: &'d Dictionary) -> Self {
        Self {
            rng: Rng::default(),
            dictionary,
        }
    }

    /// Pick `num_words` number of words from the wordbank, join them together with a single space,
    /// then return as a String.
    pub fn generate_words(&mut self, num_words: usize) -> String {
        // create a vector with a big enough allocation to hold `num_words` amount of &str
        let mut sentence = Vec::with_capacity(num_words);

        for _ in 0..num_words {
            // choose a word at random
            let word = self.rng.choose(&self.dictionary.words);

            // push the &str (pointer to the String + length) into the vector
            sentence.push(word.as_str());
        }

        // join up all those &strs into a space separated String
        sentence.join(" ")
    }
}

// Tests for the Generator type. These get run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_words() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        let mut g = Generator::with_dict(&d);
        assert_eq!("jkl", g.generate_words(1));
        assert_eq!("ghi", g.generate_words(1));
        assert_eq!("ghi", g.generate_words(1));
        assert_eq!("abc", g.generate_words(1));
        assert_eq!("abc", g.generate_words(1));
        assert_eq!("abc", g.generate_words(1));
        assert_eq!("def", g.generate_words(1));
    }

    #[test]
    fn generate_sentence() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        let mut g = Generator::with_dict(&d);
        assert_eq!("jkl ghi ghi abc abc abc def", g.generate_words(7));
    }

    #[test]
    fn clone_debug() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        let gen = Generator::with_dict(&d);
        let new_gen = gen.clone();
        println!("{:?}", new_gen);
    }
}
