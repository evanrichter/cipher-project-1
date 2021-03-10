//! Module for cracking ciphertexts.
//!
//! This module holds all code needed for cracking ciphertexts specifically encrypted using the
//! project encryption model: [`Encryptor`][`crate::ciphers::Encryptor`]

mod crack;
mod keylength;
mod spellcheck;

pub use crack::crack;
pub use keylength::guesses;
pub use spellcheck::spellcheck;

/// Every cracking strategy produces some plaintext along with a confidence value. If we run two
/// different strategies, both are successful (returning `Some(CrackResult)`), but the plaintexts
/// don't match, we could try to guess the correct one based on the confidence value.
#[derive(Clone)]
pub struct CrackResult {
    /// Guessed plaintext.
    pub plaintext: Vec<u8>,
    /// Confidence value associated with the plaintext on a scale of 0-100. Lower values correspond
    /// to **most confident** with 0.0 being the absolute most confident.
    ///
    /// An example way to calculate confidence would be to take the number of characters in words
    /// that needed to be "spell corrected" to a valid word in the dictionary, divided by the
    /// length of plaintext. This would
    pub confidence: f64,
}

#[test]
fn end_to_end() {
    use crate::ciphers;
    use crate::ciphers::schedulers::*;
    use crate::utils::*;
    use crate::{Cipher, Dictionary, Encryptor, Generator, Rng};

    let mut words = std::fs::read_to_string("words/default.txt").unwrap();
    let dict = Dictionary::from_string(&mut words);

    let mut gen = Generator::with_dict(&dict);

    let sched = PeriodicRand {
        period: 18,
        start: 3,
        overwrite: true,
    };
    let key = vec![0, 1, -1, 13, 14, -7, 9];

    let encryptor = Encryptor::new(key, sched, Rng::default());

    let plaintext = gen.generate_words(100);
    let ciphertext = encryptor.encrypt(&plaintext);
    println!("encrypted: {}", ciphertext);
    println!("plaintext:\n{}", plaintext);

    let cipherbytes = str_to_bytes(&ciphertext);
    let keylen_guesses = guesses(&cipherbytes);

    let mut crack_results = Vec::new();

    let baseline_freqs = crack::Frequencies::from_dict(&dict);
    for (keylen, _) in keylen_guesses.iter().take(15) {
        let res = crack(&cipherbytes, *keylen, &baseline_freqs);
        crack_results.push(res);
    }

    // try to be lucky
    let best = crack::best_crack(&crack_results);
    println!(
        "best crack result from known keylength:\n{}",
        bytes_to_str(&best.plaintext)
    );

    // spell check it
    let spell_checked = spellcheck(&best, &dict);
    println!("spell checked:\n{}", bytes_to_str(&spell_checked.plaintext));

    assert_eq!(bytes_to_str(&spell_checked.plaintext), plaintext);

    let decrypted = encryptor.decrypt(&ciphertext);
    println!("decrypted: {}", decrypted);
    println!();
}
