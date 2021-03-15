//! Module for cracking ciphertexts.
//!
//! This module holds all code needed for cracking ciphertexts specifically encrypted using the
//! project encryption model: [`Encryptor`][`crate::ciphers::Encryptor`]

mod crack_known_keylength;
mod keylength;
mod spellcheck;
pub mod worker;

pub use crack_known_keylength::{best_crack, crack, Frequencies};
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
    use crate::ciphers::schedulers::*;
    use crate::ciphers::{Cipher, Encryptor};
    use crate::dict::{BytesDictionary, Dictionary};
    use crate::gen::Generator;
    use crate::rng::Rng;
    use crate::utils::*;

    //
    // SETUP
    //

    let mut words = std::fs::read_to_string("words/default.txt").unwrap();
    let dict = Dictionary::from_string(&mut words);

    let mut gen = Generator::with_dict(&dict);

    let sched = PeriodicRand {
        period: 18,
        start: 5,
        overwrite: true,
    };
    let key = vec![10, 10, 12, 1, 2, 3, 4];

    let encryptor = Encryptor::new(key, sched, Rng::default());

    let plaintext = gen.generate_words(300);
    let ciphertext = encryptor.encrypt(&plaintext);
    println!("encrypted:\n{}\n", ciphertext);
    println!("plaintext:\n{}\n", plaintext);

    let cipherbytes = str_to_bytes(&ciphertext);

    //
    // KEYLENGTH GUESSING
    //

    let mut keylen_guesses = Vec::new();
    guesses(&cipherbytes, &mut keylen_guesses);

    //
    // CRACKING SLICES
    //

    let mut crack_results = Vec::new();

    let baseline_freqs = Frequencies::from_dict(&dict);

    for (keylen, _) in keylen_guesses {
        let res = crack(&cipherbytes, keylen, &baseline_freqs);
        crack_results.push(res);
    }

    let best = best_crack(&crack_results);
    println!(
        "best crack result from known keylength:\n{}\n",
        bytes_to_str(&best.plaintext)
    );

    //
    // SPELL CHECKING
    //

    let mut spell_checked = Vec::new();
    let bytesdict = BytesDictionary::from_dict(&dict);

    for crack in crack_results {
        spell_checked.push(spellcheck(&crack, &bytesdict));
    }

    let best = best_crack(&spell_checked);

    println!(
        "best crack result after spell check:\n{}\n",
        bytes_to_str(&best.plaintext)
    );

    assert_eq!(
        bytes_to_str(&best.plaintext),
        plaintext,
        "cracked and spell checked result does not match actual plaintext"
    );
}
