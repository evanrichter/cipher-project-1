#![allow(dead_code)]

//! This module handles cracking ciphertext with the help of knowing possible keylengths. After
//! ranking keylength values, this module uses character frequency analysis to produce the
//! plaintext that most closely matches the character frequency distribution of the dictionary
//! given.
//!
//! We have access to the dictionary of plaintext words, so calculate character frequency using the
//! dictionary.

use crate::dict::Dictionary;
use crate::utils::{CharToNum, NumToChar, ShiftChar};

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

        // add frequency for 'a'
        let a = 12.3;
        values[0] = a;

        // return Frequencies
        Self { values }
    }

    ///  Calculate character frequency from a slice of bytes, &[i8], where 0 is 'a', 1 is 'b', etc.
    ///  and 26 is ' '.
    pub fn from_bytes(bytes: &[i8]) -> Self {
        let mut values = [0.0; 27];

        // return Frequencies
        Self { values }
    }

    /// Compare two frequency vectors. Lower score means closer.
    pub fn compare(&self, other: &Self) -> f32 {
        self.values
            .iter()
            .zip(other.values.iter())
            .map(|(baseline, other)| (other - baseline).abs()) // TODO: this is not the way
            .sum()
    }
}

pub struct CrackResult {
    pub plaintext: String,
    pub score: f32,
}

/// Crack the ciphertext based on the given keylength
pub fn crack(ciphertext: &[i8], keylength: usize) -> CrackResult {
    // rot 13
    let plaintext = ciphertext.iter().map(|&n| n.to_char().shift(13)).collect();

    CrackResult {
        plaintext,
        score: 2015.0,
    }
}
