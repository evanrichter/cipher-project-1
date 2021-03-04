//! Module for [`Generator`].

use crate::dict::Dictionary;
use crate::rng::Rng;

/// A deterministic plaintext generator. The purpose is to be able to quickly generate known
/// plaintexts so that we can encipher them, and then attempt to crack the ciphertext. Since we
/// generated the plaintext ourself, we can simply compare our cracking results to verify.
#[derive(Clone, Debug)]
pub struct Generator<'d> {
    dictionary: &'d Dictionary<'d>,
    pub rng: Rng,
}

impl<'d> Generator<'d> {
    /// Instantiate a generator that generates messages using the given [`Dictionary`] as a
    /// wordbank.
    pub fn with_dict(dictionary: &'d Dictionary<'d>) -> Self {
        Self {
            rng: Rng::default(),
            dictionary,
        }
    }

    /// Pick `num_words` number of words from the wordbank, join them together with a single space,
    /// then return as a String.
    pub fn generate_words(&mut self, num_words: usize) -> String {
        let mut sentence = String::new();
        self.generate_words_into(num_words, &mut sentence);
        sentence
    }

    /// Same as [`generate_words`] but appends to a String rather than returning a String. This may
    /// be a good option for optimizations to reduce allocation.
    pub fn generate_words_into(&mut self, num_words: usize, dest: &mut String) {
        // prepend a space if we are appending to an already existing sentence
        if dest.len() > 0 && !dest.ends_with(" ") {
            dest.push(' ');
        }

        for _ in 0..num_words {
            // choose a word at random
            let word = *self.rng.choose(&self.dictionary.words);

            // append the &str's characters to the String
            dest.extend(word.chars());

            // append a space
            dest.push(' ');
        }

        // pop off the last trailing space if we added words
        if num_words > 0 {
            dest.pop();
        }
    }
}

// Tests for the Generator type. These get run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_words() {
        let mut s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(&mut s);

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
        let mut s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(&mut s);

        let mut g = Generator::with_dict(&d);
        assert_eq!("jkl ghi ghi abc abc abc def", g.generate_words(7));
    }

    #[test]
    fn clone_debug() {
        let mut s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(&mut s);

        let gen = Generator::with_dict(&d);
        let new_gen = gen.clone();
        println!("{:?}", new_gen);
    }
}
