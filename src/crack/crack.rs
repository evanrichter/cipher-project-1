#![allow(dead_code)]

//! This module handles cracking ciphertext with the help of knowing possible keylengths. After
//! ranking keylength values, this module uses character frequency analysis to produce the
//! plaintext that most closely matches the character frequency distribution of the dictionary
//! given.
//!
//! We have access to the dictionary of plaintext words, so calculate character frequency using the
//! dictionary.

use core::f32;
use std::{collections::HashMap, convert::TryInto};
use std::collections::hash_map::Entry;

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
        // open file and read contents to string
        //let mut file = File::open("/path/to/file").expect("Unable to open the file");
        let mut s = String::new();
        //file.read_to_string(&mut s).expect("Unable to read the file");
        
        let mut values = [0.0; 27];
        let mut h: HashMap<char, f32> = HashMap::new();

        // generate a hashmap to get the occurance of every character in the string
        for c in s.chars() {
            match h.entry(c) {
                Entry::Occupied(mut x) => {*x.get_mut() += 1.0;}
                Entry::Vacant(_) => {h.insert(c,1.0);}
            }
        }
        let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz ");
        // add frequency for 'a'
        for i in 0..=26 {
            match h.get(&alphabet.chars().nth(i).unwrap()) {
                Some(h) => {values[i] = *h;}
                None => {continue;}
            }
        }

        //this is how i'd like to do this:
        for i in 0..=26 {
            //let mut num = i as i8;
            match h.get(i.NumToChar()) {
                Some(h) => {values[i] = *h;}
                None => {continue;}
            }
        }
        

        // return Frequencies
        Self { values }
    }

    ///  Calculate character frequency from a slice of bytes, &[i8], where 0 is 'a', 1 is 'b', etc.
    ///  and 26 is ' '.
    pub fn from_bytes(bytes: &[i8]) -> Self {
        let mut values = [0.0; 27];
        let mut h: HashMap<char, usize> = HashMap::new();
        
        // I *think* this will just be the same as above?
        // generate a hashmap to get the occurance of every character in the string
        for c in s.chars() {
            match h.entry(c) {
                Entry::Occupied(mut x) => {*x.get_mut() += 1;}
                Entry::Vacant(_) => {h.insert(c,1);}
            }
        }

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
        
        self.values
            .iter()
            .zip(other.values.iter())
            
    
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
