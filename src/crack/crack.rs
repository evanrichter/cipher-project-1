#![allow(dead_code)]

//! This module handles cracking ciphertext with the help of knowing possible keylengths. After
//! ranking keylength values, this module uses character frequency analysis to produce the
//! plaintext that most closely matches the character frequency distribution of the dictionary
//! given.
//!
//! We have access to the dictionary of plaintext words, so calculate character frequency using the
//! dictionary.

use crate::utils::{NumToChar, ShiftChar};
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

        return sum_of_differences;
    }
}

/// Every cracking strategy produces some plaintext along with a confidence value. If we run two
/// different strategies, both are successful (returning `Some(CrackResult)`), but the plaintexts
/// don't match, we could try to guess the correct one based on the confidence value.
#[derive(Clone)]
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

/// Return the best (smallest confidence value) CrackResult from a list of many
pub fn best_crack(crackresults: &[CrackResult]) -> CrackResult {
    assert!(crackresults.len() > 0);
    crackresults
        .iter()
        .min_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap()) // have to unwrap because floats can be NaN (but should not happen to us)
        .unwrap() // only could be None if iterator is empty
        .clone()
}

/// Slice ciphertext into chunks of every (keylength) character
pub fn slice(ciphertext: &[i8], keylength: usize) -> Vec<Vec<i8>> {
    let mut ct_blocks = vec![];
    for i in 0..keylength {
        let block: Vec<_> = ciphertext
            .iter()
            .skip(i) // first skip past the first i items
            .step_by(keylength) // now skip by a keylength every time
            .copied() // copies the &i8 item type to i8 (owned)
            .collect();
        ct_blocks.push(block);
    }

    // should have exactly keylength number of blocks
    debug_assert_eq!(
        keylength,
        ct_blocks.len(),
        "need same number chunks as indices in keylength!"
    );

    // make sure we didn't lose any ciphertext chars
    debug_assert_eq!(
        ciphertext.len(),
        ct_blocks.iter().map(|block| block.len()).sum(),
        "total chars same as original ciphertext length!"
    );

    ct_blocks
}

/// Unslice the highest confidence plaintext into a normal string
/// TODO: change this and other intermediate cracking steps where we handle Strings like this to
/// Vec<i8> and &[i8]
pub fn unslice(pt_blocks: Vec<String>, keylength: usize) -> String {
    // get a single string ready
    let mut unsliced = String::with_capacity(pt_blocks[0].len() * keylength);

    // the first block will be the longest, so we iterate over indexes of that
    for i in 0..pt_blocks[0].chars().count() {
        // for every index, go through each block and pull out the character there.
        // if there is no character there, just continue
        for block in pt_blocks.iter() {
            // this here is awkward because we can't simply do block.get(i) on a str (because of
            // unicode) this will be better after changing to only handling i8 instead of char and
            // str when cracking
            if let Some(c) = block.chars().nth(i) {
                unsliced.push(c);
            }
        }
    }

    unsliced
}

/// Crack a single block of ciphertext as if it were shifted with a key of length 1
fn crack_block(cipherblock: &[i8], baseline: &Frequencies) -> CrackResult {
    // vector to hold each individual shift attempt
    let mut crack_results: Vec<CrackResult> = Vec::with_capacity(27);

    // try each shift in the alphabet (0 shift == 27 shift)
    for shift in 0..ALPHABET.len() as i8 {
        // make the plaintext
        let plaintext: String = cipherblock
            .iter()
            .map(|&n| n.to_char().shift(shift))
            .collect();
        // calculate the confidence to baseline
        let confidence = Frequencies::compare(baseline, &Frequencies::from_str(&plaintext)) as f64;
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
pub fn crack(ciphertext: &[i8], keylength: usize, baseline: &Frequencies) -> CrackResult {
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
    let pt_chunks: Vec<String> = crack_results
        .iter()
        .map(|cr| cr.plaintext.clone())
        .collect();
    let plaintext: String = unslice(pt_chunks, keylength);

    // confidence overall is sum of each individual confidence
    let total_confidence = crack_results.iter().map(|cr| cr.confidence).sum();

    CrackResult {
        plaintext,
        confidence: total_confidence,
    }
}
