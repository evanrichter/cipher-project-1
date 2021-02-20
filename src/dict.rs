//! Module for [`Dictionary`].

/// A dictionary will hold an alphabetized wordlist. Each word only consists of lowercase ASCII
/// alphabetic characters.
#[derive(Clone, Debug)]
pub struct Dictionary {
    pub words: Vec<String>,
}

impl Dictionary {
    /// Create a dictionary from a single string that is whitespace  separated. This function will
    /// work for both:
    ///  * the project specified dictionary input (a file with space separated words)
    ///  * the common file format for words files (newline separated words)
    ///
    ///  This function also rejects any word that contains non alphabetic ascii characters,
    ///  printing to stderr for each word it tosses out.
    pub fn from_string(source: String) -> Self {
        let mut words: Vec<String> = source
            // trim off starting and trailing whitespace
            .trim()
            // split by any type of ascii whitespace
            .split_ascii_whitespace()
            // make sure the word is only a-zA-Z
            .filter(|word| {
                let alphabetic = word.chars().all(|chr| chr.is_alphabetic());
                if !alphabetic {
                    eprintln!("word \"{}\" is non-alphabetic", word);
                }
                alphabetic
            })
            // lowercase the word
            .map(|word| word.to_ascii_lowercase())
            // call next on the iterator, building up a single Vec
            .collect();

        // sort the words alphabetically
        words.sort();

        // return the dictionary
        Self { words }
    }

    /// Return how many words are in the dictionary
    pub fn len(&self) -> usize {
        self.words.len()
    }

    /// Find the closest word by Levenshtein distance.
    ///
    /// Returns (dictionary_word, edit_distance)
    ///
    /// The lower the score, the fewer edits needed to match the dictionary word.
    pub fn best_levenshtein(&self, word: &str) -> (&str, usize) {
        // iterate over words in dictionary
        self.words
            .iter()
            .map(|s| s.as_str())
            // create tuples of &str and the respective levenshtein distance
            .map(|s| (s, strsim::levenshtein(word, s)))
            // return the best word-score tuple
            .min_by_key(|x| x.1)
            .expect("spell correct with an empty Dictionary")
    }
}

// Tests for the Dictionary type. These get run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "ghi");
        assert_eq!(d.words[3], "jkl");
    }

    #[test]
    fn len() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words.len(), 4);
    }

    #[test]
    fn reject_nonascii() {
        let s = String::from("abc def g.hi jkl");
        let d = Dictionary::from_string(s);

        assert_eq!(d.len(), 3);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "jkl");
    }

    #[test]
    fn order() {
        let s = String::from("def jkl abc ghi");
        let d = Dictionary::from_string(s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "ghi");
        assert_eq!(d.words[3], "jkl");
    }

    #[test]
    fn trim() {
        let s = String::from("    abc \n  def \t ghi   jkl\n\n  ");
        let d = Dictionary::from_string(s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "ghi");
        assert_eq!(d.words[3], "jkl");
    }

    #[test]
    fn levenshtein() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        assert_eq!(d.best_levenshtein("acb"), ("abc", 2));
        assert_eq!(d.best_levenshtein("de"), ("def", 1));
        assert_eq!(d.best_levenshtein("ghi"), ("ghi", 0));
        assert_eq!(d.best_levenshtein(" jkl "), ("jkl", 2));
        assert_eq!(d.best_levenshtein("abc def"), ("abc", 4));
    }
}
