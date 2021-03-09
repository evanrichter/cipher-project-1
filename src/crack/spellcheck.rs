//! Module for correcting nearly perfect plaintext, into a plausible plaintext that actually could
//! have been generated from the source dictionary.

use core::f64;

use strsim::levenshtein;

use super::CrackResult;
use crate::{utils::bytes_to_str, utils::str_to_bytes, Dictionary};

/// This function exploits the fact that we know the source dictionary (or can guess between a
/// small number of dictionaries), and uses spell checking strategies to fix up any incorrectly
/// guessed shift values from the previous step.

#[allow(dead_code)]
pub fn spellcheck(cracked: &CrackResult, dict: &Dictionary) -> CrackResult {
    let mut correcttext: String; //the string we will correct
    let iteratortext: String; //this is used on the for loop as an iterator

    //convert from bytes to str
    iteratortext = bytes_to_str(&cracked.plaintext);

    correcttext = bytes_to_str(&cracked.plaintext).to_string();

    //compare words to dict and correct inaccuracies
    for word in iteratortext.split_whitespace()
    //split_whitespace chunks the string out into words and returns an iterator
    {
        let dict_match = dict.best_levenshtein(word); //finds the closest word match between the dict and our plaintext result

        if dict_match.1 == 0
        //if its an exact match, we do nothing
        {
            continue;
        } else
        //if it isn't, we make a change
        {
            correcttext = correcttext.replace(word, dict_match.0);
        }
    }

    //caclulate the confidence
    let numberofdif = levenshtein(&bytes_to_str(&cracked.plaintext), &correcttext) as f64; //finds the number of changes made between the corrected and origional
    let newconfidence: f64 = numberofdif / cracked.plaintext.len() as f64; //takes the number of levenstein differences over hte length. THis is the ratio between changes made and the length of the text. the higher the value the lower the confidence

    //create the output
    let errorcorrect = CrackResult {
        plaintext: str_to_bytes(&correcttext),
        confidence: newconfidence,
    };
    //final result
    errorcorrect
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        let newstring=String::from("wordss wishes this pig the quics brown fox jumpede over the lazy dog cat lion seal fish canary sf f a fish carp sharks");
        let cracked = CrackResult {
            plaintext: str_to_bytes(&newstring),
            confidence: 4000.0,
        };

        let targetplaintext=String::from("words wishes that pig the quick brown fox jumped over the lazy dog cat lion seal fish canary sf f a fish carp shark");
        let bytestarget = str_to_bytes(&targetplaintext);
        let dict = Dictionary {
            words: [
                "words", "wards", "wishes", "that", "pig", "the", "quick", "brown", "fox",
                "jumped", "over", "the", "lazy", "dog", "cat", "lion", "seal", "fish", "canary",
                "sf", "f", "a", "fosh", "carp", "shark", "pie", "sandle", "counter", "keyboard",
                "airplane", "fresh", "wishes",
            ]
            .to_vec(),
        };
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
