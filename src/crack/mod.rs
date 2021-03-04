//! Module for cracking ciphertexts.
//!
//! This module holds all code needed for cracking ciphertexts specifically encrypted using the
//! project encryption model: [`Encryptor`][`crate::ciphers::Encryptor`]

mod keylength;

pub use keylength::guesses;

/// Every cracking strategy produces some plaintext along with a confidence value. If we run two
/// different strategies, both are successful (returning `Some(CrackResult)`), but the plaintexts
/// don't match, we could try to guess the correct one based on the confidence value.
#[allow(dead_code)]
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

/// Trait associated with cracking ciphertexts.
pub trait Crack {
    /// Attempts to crack the given ciphertext. Returns [`Some(CrackResult)`][`CrackResult`] when a
    /// plaintext could be recovered, or [`None`] if not.
    fn crack(&self, ciphertext: String) -> Option<CrackResult>;
}
