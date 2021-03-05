#![allow(dead_code)]

//! This module handles cracking ciphertext with the help of knowing possible keylengths. After
//! ranking keylength values, this module uses character frequency analysis to produce the
//! plaintext that most closely matches the character frequency distribution of the dictionary
//! given.
//!
//! We have access to the dictionary of plaintext words, so calculate character frequency using the
//! dictionary.

use std::convert::TryInto;

use crate::dict::Dictionary;
use crate::utils::{NumToChar, ShiftChar};


/// Frequency distribution
pub struct Frequencies {
    /// values[0]  => frequency of 'a'
    /// values[1]  => frequency of 'b'
    ///           ....
    /// values[25] => frequency of 'z'
    /// values[26] => frequencty of ' '
    values: [f32; 27],
}

impl Frequencies {
    ///  Generate the baseline character frequency from the given dictionary.
    pub fn from_dict(dict: &Dictionary) -> Self {
        let mut values = [0.0; 27];

        // count occurrences of all letters except space
        for (index, letter) in crate::utils::ALPHABET.chars().enumerate().take(26) {
            let mut count = 0;
            for word in &dict.words {
                count += word.chars().filter(|c| c == &letter).count();
            }
            values[index] = count as f32;
        }

        // for space, every word is followed by a space, so we can just count words
        values[26] = dict.words.len() as f32;

        // divide each letter count by the total to get a fraction
        let total: f32 = values.iter().sum();
        for v in values.iter_mut() {
            *v = *v / total;
        }

        // return Frequencies
        Self { values }
    }

    ///  Calculate character frequency from a slice of bytes, &[i8], where 0 is 'a', 1 is 'b', etc.
    ///  and 26 is ' '.
    pub fn from_bytes(bytes: &[i8]) -> Self {
        let mut values = [0.0; 27];
       
        // the byte values are assumed to already be "nice" and in the range 0-26. Rust will crash
        // safely if this is not the case.
        //
        // the utils::str_to_bytes function should be used early on when using bytes instead of
        // chars so this is ok.
        for b in bytes {
            values[*b as usize] += 1.0;
        }

        // divide by total number of bytes
        for v in values.iter_mut() {
            *v = *v / (bytes.len() as f32);
        }

        // return Frequencies
        Self { values }
    }

    /// Compare two frequency vectors. Lower score means closer.
    pub fn compare(&self, other: &Self) -> f32 {
        let sum_of_differences = self.values
            .iter()
            .zip(other.values.iter())
            .map(|(baseline, other)| (other - baseline).abs()) // TODO: this is not the way
            .sum();
       
        /*
        self.values
            .iter()
            .zip(other.values.iter())
            
    
        }
        */
        return sum_of_differences;
}
}

/// Every cracking strategy produces some plaintext along with a confidence value. If we run two
/// different strategies, both are successful (returning `Some(CrackResult)`), but the plaintexts
/// don't match, we could try to guess the correct one based on the confidence value.
pub struct CrackResult {
    /// Guessed plaintext.
    pub plaintext: String,
    /// Confidence value associated with the plaintext on a scale of 0-100. Lower values correspond
    /// to **most confident** with 0.0 being the absolute most confident.
    ///
    /// An example way to calculate confidence would be to take the number of characters in words
    /// that needed to be "spell corrected" to a valid word in the dictionary, divided by the
    /// length of plaintext. This would
    pub confidence: f64,
}

/// Crack the ciphertext based on the given keylength
pub fn crack(ciphertext: &[i8], keylength: usize) -> CrackResult {
    // rot 13
    //let plaintext = ciphertext.iter().map(|&n| n.to_char().shift(13)).collect();

    let plaintext = ciphertext.iter().map(|&n| n.to_char().shift(keylength.try_into().unwrap())).collect();



    CrackResult {
        plaintext,
        confidence: 2015.0,
    }
}
