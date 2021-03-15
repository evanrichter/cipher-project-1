#![allow(dead_code)]

//! This module handles cracking ciphertext with the help of knowing possible keylengths. After
//! ranking keylength values, this module uses character frequency analysis to produce the
//! plaintext that most closely matches the character frequency distribution of the dictionary
//! given.
//!
//! We have access to the dictionary of plaintext words, so calculate character frequency using the
//! dictionary.

use super::CrackResult;
use crate::utils::Shift;
use crate::{
    dict::Dictionary,
    utils::{str_to_bytes, ALPHABET},
};

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
        for (index, letter) in ALPHABET.chars().enumerate().take(26) {
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
            *v /= total;
        }

        // return Frequencies
        Self { values }
    }

    ///  Calculate character frequency from a slice of bytes, &[u8], where 0 is 'a', 1 is 'b', etc.
    ///  and 26 is ' '.
    pub fn from_bytes(bytes: &[u8]) -> Self {
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
            *v /= bytes.len() as f32;
        }

        // return Frequencies
        Self { values }
    }

    pub fn from_str(s: &str) -> Self {
        Self::from_bytes(str_to_bytes(s).as_slice())
    }

    /// Compare two frequency vectors. Lower score means closer.
    pub fn compare(&self, other: &Self) -> f32 {
        let sum_of_differences = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(baseline, other)| (other - baseline).abs()) // TODO: this is not the way
            .sum();

        sum_of_differences
    }
}

/// Return the best (smallest confidence value) CrackResult from a list of many
pub fn best_crack(crackresults: &[CrackResult]) -> CrackResult {
    assert!(!crackresults.is_empty());
    crackresults
        .iter()
        .min_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap()) // have to unwrap because floats can be NaN (but should not happen to us)
        .unwrap() // only could be None if iterator is empty
        .clone()
}

/// Slice ciphertext into chunks of every (keylength) character
pub fn slice(ciphertext: &[u8], keylength: usize) -> Vec<Vec<u8>> {
    let mut ct_blocks = vec![Vec::new(); keylength];

    // pass over ciphertext, dumping the char into the right bucket as we go
    for (ct_index, ct_char) in ciphertext.iter().enumerate() {
        // find which bucket to use
        let bucket = ct_index % keylength;

        // push the char into that bucket
        ct_blocks[bucket].push(*ct_char);
    }

    ct_blocks
}

/// Unslice the highest confidence plaintext into a normal string
pub fn unslice(pt_blocks: Vec<Vec<u8>>, keylength: usize) -> Vec<u8> {
    // get a single string ready
    let mut unsliced = Vec::with_capacity(pt_blocks[0].len() * keylength);

    // the first block will be the longest, so we iterate over indexes of that
    for i in 0..pt_blocks[0].len() {
        // for every index, go through each block and pull out the character there.
        // if there is no character there, just continue
        for block in pt_blocks.iter() {
            if let Some(c) = block.get(i) {
                unsliced.push(*c);
            }
        }
    }

    unsliced
}

/// Crack a single block of ciphertext as if it were shifted with a key of length 1
fn crack_block(cipherblock: &[u8], baseline: &Frequencies) -> CrackResult {
    // vector to hold each individual shift attempt
    let mut crack_results: Vec<CrackResult> = Vec::with_capacity(27);

    // try each shift in the alphabet (0 shift == 27 shift)
    for shift in 0..ALPHABET.len() as i8 {
        // make the plaintext
        let plaintext: Vec<u8> = cipherblock.iter().map(|&n| n.shift(shift)).collect();

        // calculate the confidence to baseline
        let confidence =
            Frequencies::compare(baseline, &Frequencies::from_bytes(&plaintext)) as f64;

        // push the result
        crack_results.push(CrackResult {
            plaintext,
            confidence,
        });
    }

    // return the best result
    best_crack(&crack_results)
}

/// Crack the ciphertext based on the given keylength
pub fn crack(ciphertext: &[u8], keylength: usize, baseline: &Frequencies) -> CrackResult {
    // slice up the ciphertext based on keylength
    let ct_blocks = slice(ciphertext, keylength);

    // vector to store crackresults. we will get one result from each index of the keylength so we
    // allocate for that number of items up front.
    let mut crack_results: Vec<CrackResult> = Vec::with_capacity(keylength);

    // crack each ct_block as if it were single key shift
    for block in ct_blocks {
        crack_results.push(crack_block(&block, baseline));
    }

    // de-interleave the plaintext chunks back into one contiguous plaintext
    let pt_chunks: Vec<Vec<u8>> = crack_results
        .iter()
        .map(|cr| cr.plaintext.clone())
        .collect();
    let plaintext: Vec<u8> = unslice(pt_chunks, keylength);

    // confidence overall is sum of each individual confidence
    let total_confidence = crack_results.iter().map(|cr| cr.confidence).sum();

    CrackResult {
        plaintext,
        confidence: total_confidence,
    }
}
