//! Definition of [`KeySchedule`] and various implementations of key scheduling.

mod aab;
mod invertZip;
mod repeatingkey;

pub use aab::Aab;
pub use invertZip::InvertZip;
pub use repeatingkey::RepeatingKey;

/// Trait for implementing key scheduling.
pub trait KeySchedule {
    /// Returns the index of the key to use when shifting plaintext into ciphertext.
    ///
    /// In the project description, this process is described as: each ciphertext symbol `c[i]` is the
    /// shift of the plaintext symbol `m[i]` by a number of position equal to one of the key symbols,
    /// which symbol being chosen according to an _undisclosed, deterministic, and not key-based_,
    /// scheduling algorithm that is a function of `i`, `t` and `L`, where:
    ///   * `i` is the index being output to ciphertext
    ///   * `t` is the key length
    ///   * `L` is the length of the plaintext
    fn schedule(&self, index: usize, key_length: usize, plaintext_length: usize) -> usize;
}
