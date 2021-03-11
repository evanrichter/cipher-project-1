//! Module for correcting nearly perfect plaintext, into a plausible plaintext that actually could
//! have been generated from the source dictionary.

use super::CrackResult;
use crate::dict::{levenshtein, BytesDictionary};

use std::cmp::min;

struct Word<'a> {
    word: &'a [u8],
    score: usize,
    bytes_used: usize,
}

impl<'a> Word<'a> {
    // higher score is better
    //
    // prefer longer words and smaller edit-distance
    fn score(&self) -> usize {
        (self.bytes_used as f32 / self.score as f32 * 1000.0) as usize
    }
}

/// This function exploits the fact that we know the source dictionary (or can guess between a
/// small number of dictionaries), and uses spell checking strategies to fix up any incorrectly
/// guessed shift values from the previous step.
#[allow(dead_code)]
pub fn spellcheck(cracked: &CrackResult, dict: &BytesDictionary) -> CrackResult {
    //the string we will correct
    let mut plaintext: Vec<u8> = Vec::with_capacity(cracked.plaintext.len());

    // the longest word in the dictionary given
    //
    // I don't know why it needs a +1 ...
    let longest_word = dict.words.iter().map(|w| w.len()).max().unwrap() + 1;

    // a slice where the start is always pointing to the next word to spell check, and the end goes
    // all the way to the end of the given plaintext.
    let mut next_slice = cracked.plaintext.as_slice();

    // temporary vec to hold scores for scanned words
    let mut next_words: Vec<Word> = Vec::new();

    while next_slice.len() > 1 {
        // farthest right to try to match
        let rbound = min(longest_word, next_slice.len());

        // find the next possible words
        for bytes_used in 1..rbound {
            let (word, score) = dict.best_levenshtein(&next_slice[..bytes_used]);
            let word = Word {
                word,
                score,
                bytes_used,
            };
            next_words.push(word);
        }

        // pick the best word from next_words
        let best = next_words.iter().max_by_key(|word| word.score()).unwrap();

        // add the best word to the plaintext
        plaintext.extend_from_slice(best.word);

        // advance to the next word by however many characters we read
        next_slice = &next_slice[best.bytes_used..];

        // clear the next_words vec
        next_words.clear();
    }

    // pop off the last space because all dictionary words come with a space
    plaintext.pop();

    // overall confidence is levenshtein edit distance from what we recovered to the given
    // near-plaintext. (Not sure how useful this is...)
    let confidence = levenshtein(&plaintext, &cracked.plaintext) as f64;

    CrackResult {
        plaintext,
        confidence: confidence * cracked.confidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dict::Dictionary;
    use crate::utils::*;

    #[test]
    fn testing() {
        let newstring=String::from("wordss wishes this pig the quics brown fox jumpede over the lazy dog cat lion seal fish canary sf f a fish carp sharks");
        let cracked = CrackResult {
            plaintext: str_to_bytes(&newstring),
            confidence: 4000.0,
        };

        let targetplaintext=String::from("words wishes that pig the quick brown fox jumped over the lazy dog cat lion seal fish canary sf f a fish carp shark");
        let bytestarget = str_to_bytes(&targetplaintext);
        let dict = BytesDictionary::from_dict(&Dictionary {
            words: [
                "words", "wards", "wishes", "that", "pig", "the", "quick", "brown", "fox",
                "jumped", "over", "the", "lazy", "dog", "cat", "lion", "seal", "fish", "canary",
                "sf", "f", "a", "fosh", "carp", "shark", "pie", "sandle", "counter", "keyboard",
                "airplane", "fresh", "wishes",
            ]
            .to_vec(),
        });
        // cracked.plaintext = "wards wishes this pig the quics brown fox jumpede over the lazy dog cat lion seal fish canary sf f a fash carp sharks".to_string();

        println!(
            "BEFORE TEST plaintext is  {}\n",
            bytes_to_str(&cracked.plaintext)
        );

        let errorcorrect = spellcheck(&cracked, &dict);

        println!(
            "AFTER TEST Plaintext is  {}\n",
            bytes_to_str(&errorcorrect.plaintext)
        );

        assert_eq!(&errorcorrect.plaintext, &bytestarget);
    }
}
