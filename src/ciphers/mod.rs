//! Implementations of various ciphers.

mod rot13;
pub use rot13::Rot13;

use crate::Dictionary;

/// The Cipher trait describes what every cipher needs to be able to do.
pub trait Cipher {
    fn encrypt<'d>(&self, plaintext: &str, dict: &'d Dictionary) -> String;
    fn decrypt<'d>(&self, ciphertext: &str, dict: &'d Dictionary) -> String;
}
