//! Module for cracking ciphertexts.
//!
//! This module holds all code needed for cracking ciphertexts specifically encrypted using the
//! project encryption model: [`Encryptor`][`crate::ciphers::Encryptor`]

mod crack;
mod keylength;

pub use crack::{crack, CrackResult};
pub use keylength::guesses;
