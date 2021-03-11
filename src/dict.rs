//! Module for [`Dictionary`].

use crate::utils::str_to_bytes;

/// A dictionary will hold an alphabetized wordlist. Each word only consists of lowercase ASCII
/// alphabetic characters.
#[derive(Clone, Debug)]
pub struct Dictionary<'a> {
    pub words: Vec<&'a str>,
}

impl<'a> Dictionary<'a> {
    /// Create a dictionary from a single string that is whitespace  separated. This function will
    /// work for both:
    ///  * the project specified dictionary input (a file with space separated words)
    ///  * the common file format for words files (newline separated words)
    ///
    ///  This function also rejects any word that contains non alphabetic ascii characters,
    ///  printing to stderr for each word it tosses out.
    pub fn from_string(source: &'a mut String) -> Self {
        // lowercase the whole source string
        *source = source.to_ascii_lowercase();

        let mut words: Vec<&str> = source
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
            // call next on the iterator, building up a single Vec
            .collect();

        // sort the words alphabetically
        words.sort();

        // return the dictionary
        Self { words }
    }
}

pub struct BytesDictionary {
    pub words: Vec<Vec<u8>>,
}

impl BytesDictionary {
    pub fn from_dict<'a>(dict: &Dictionary<'a>) -> Self {
        use crate::utils::CharToNum;

        let words = dict
            .words
            .iter()
            .map(|w| {
                let mut w = str_to_bytes(w);
                w.push(' '.to_num());
                w
            })
            .collect();

        Self { words }
    }

    /// Find the closest word by Levenshtein distance.
    ///
    /// Returns (dictionary_word, edit_distance)
    ///
    /// The lower the score, the fewer edits needed to match the dictionary word.
    pub fn best_levenshtein<'a>(&'a self, word: &[u8]) -> (&'a [u8], usize) {
        // iterate over words in dictionary
        self.words
            .iter()
            // create tuples of &str and the respective levenshtein distance
            .map(|s| (s.as_slice(), levenshtein(word, s)))
            // return the best word-score tuple
            .min_by_key(|x| x.1)
            .expect("spell correct with an empty Dictionary")
    }
}

pub fn levenshtein<'a, 'b, Iter1: ?Sized, Iter2: ?Sized, Elem1, Elem2>(
    a: &'a Iter1,
    b: &'b Iter2,
) -> usize
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    use std::cmp::min;

    let b_len = b.into_iter().count();

    if a.into_iter().next().is_none() {
        return b_len;
    }

    let mut cache: Vec<usize> = (1..b_len + 1).collect();

    let mut result = 0;

    for (i, a_elem) in a.into_iter().enumerate() {
        result = i + 1;
        let mut distance_b = i;

        for (j, b_elem) in b.into_iter().enumerate() {
            let cost = if a_elem == b_elem { 0 } else { 1 };
            let distance_a = distance_b + cost;
            distance_b = cache[j];
            result = min(result + 1, min(distance_a, distance_b + 1));
            cache[j] = result;
        }
    }

    result
}

// Tests for the Dictionary type. These get run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let mut s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(&mut s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "ghi");
        assert_eq!(d.words[3], "jkl");
    }

    #[test]
    fn len() {
        let mut s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(&mut s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words.len(), 4);
    }

    #[test]
    fn reject_nonascii() {
        let mut s = String::from("abc def g.hi jkl");
        let d = Dictionary::from_string(&mut s);

        assert_eq!(d.len(), 3);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "jkl");
    }

    #[test]
    fn order() {
        let mut s = String::from("def jkl abc ghi");
        let d = Dictionary::from_string(&mut s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "ghi");
        assert_eq!(d.words[3], "jkl");
    }

    #[test]
    fn trim() {
        let mut s = String::from("    abc \n  def \t ghi   jkl\n\n  ");
        let d = Dictionary::from_string(&mut s);

        assert_eq!(d.len(), 4);
        assert_eq!(d.words[0], "abc");
        assert_eq!(d.words[1], "def");
        assert_eq!(d.words[2], "ghi");
        assert_eq!(d.words[3], "jkl");
    }

    #[test]
    fn levenshtein() {
        let mut s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(&mut s);

        assert_eq!(d.best_levenshtein("acb"), ("abc", 2));
        assert_eq!(d.best_levenshtein("de"), ("def", 1));
        assert_eq!(d.best_levenshtein("ghi"), ("ghi", 0));
        assert_eq!(d.best_levenshtein(" jkl "), ("jkl", 2));
        assert_eq!(d.best_levenshtein("abc def"), ("abc", 4));
    }
}
