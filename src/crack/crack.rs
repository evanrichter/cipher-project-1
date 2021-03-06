#![allow(dead_code)]

//! This module handles cracking ciphertext with the help of knowing possible keylengths. After
//! ranking keylength values, this module uses character frequency analysis to produce the
//! plaintext that most closely matches the character frequency distribution of the dictionary
//! given.
//!
//! We have access to the dictionary of plaintext words, so calculate character frequency using the
//! dictionary.

use std::{convert::TryInto, string};

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

/// Slice ciphertext into chunks of every (keylength) character
pub fn slice(ciphertext: &[i8], keylength: usize) -> Vec<Vec<i8>> {
    let ct_blocks = vec![];
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
pub fn unslice(sliced_pt: String, keylength: usize)  -> String{
    let mut pt_blocks:Vec<char> = vec![];
    for i in 0..keylength {
        let mut block = vec![];
        for c in sliced_pt.chars() {
            if i % keylength == i {
                block.push(c);
            }
        }
        pt_blocks.append(&mut block);
    }
    let plaintext = pt_blocks.iter().collect();
    plaintext
}

/// Crack the ciphertext based on the given keylength
pub fn crack(ciphertext: &[i8], keylength: usize, baseline: &Frequencies) -> CrackResult {
    // rot 13
    //let plaintext = ciphertext.iter().map(|&n| n.to_char().shift(13)).collect();

    //let pt_blocks = ciphertext.iter().map(|&n| n.to_char().shift(keylength.try_into().unwrap())).collect();
    let pt_slices = vec![];
    let ct_blocks = slice(ciphertext, keylength);
    let mut total_conf: Vec<f32> = vec![];
    for block in ct_blocks {
        let mut conf_vec: Vec<f32> = vec![];
        let mut pt_block = "";
        for shift in 0..26 {
            pt_block = block.iter().map(|&n| n.to_char().shift(shift)).collect();
            let conf = Frequencies::compare(baseline,&Frequencies::from_bytes(pt_block));
            conf_vec.append(conf);
        }
        let best: f32 = conf_vec.iter().min();
        total_conf.push(best);
        pt_slices.push(pt_block);
    }

    let confidence = &total_conf.iter().sum();
    let sliced_pt: String = pt_slices.iter().collect();
    let plaintext: String = unslice(sliced_pt, keylength);

    
    



    CrackResult {
        plaintext,
        confidence: *confidence,
    }
}
