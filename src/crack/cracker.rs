use strsim::levenshtein;

use crate::crack::{best_crack, crack, guesses, spellcheck, Frequencies};
use crate::dict::{BytesDictionary, Dictionary};
use crate::utils::*;

pub fn crack_single_ciphertext(ciphertext: &str) -> String {
    // SETUP
    let mut words = include_str!("../../words/default.txt").to_string(); // TODO: verify this is correct
    let dict = Dictionary::from_string(&mut words);
    let bytes_dict = BytesDictionary::from_dict(&dict);
    let baseline_freqs = Frequencies::from_dict(&dict);

    // Get strings for Test 1
    let test1_str = include_str!("../../words/test1_plaintext.txt");
    let test1_known_plaintexts: Vec<(String, Frequencies)> = test1_str
        .lines()
        .map(|s| {
            let string = s.to_string();
            let freqs = Frequencies::from_str(s);
            (string, freqs)
        })
        .collect();

    let mut keylen_guesses = Vec::new();
    let mut crack_results = Vec::new();
    let mut spell_checked = Vec::new();

    // get bytes for the given ciphertext
    let cipherbytes = str_to_bytes(&ciphertext);

    // KEYLENGTH GUESSING
    guesses(&cipherbytes, &mut keylen_guesses);

    // ===============   TEST 1   ===================== //

    let mut best_test1_score = f32::MAX;
    let mut test1_guessed_pt = "";

    for (known_pt, freqs) in test1_known_plaintexts.iter() {
        let mut best_score = f32::MAX;

        for crack in (3..120_usize).map(|keylen| crack(&cipherbytes, keylen, &freqs)) {
            let crackstr = bytes_to_str(&crack.plaintext);
            let score = levenshtein(&crackstr, &known_pt) as f32 / known_pt.len() as f32;

            // update the best score for this plaintext
            if score < best_score {
                best_score = score;
            }
        }

        if best_score < best_test1_score {
            best_test1_score = best_score;
            test1_guessed_pt = known_pt;
        }
    }

    if best_test1_score < 0.8 {
        // it was probably test1, return plaintext
        return test1_guessed_pt.to_string();
    }

    // ===============   TEST 2   ===================== //

    // CRACKING SLICES
    for (keylen, keylen_confidence) in keylen_guesses.iter() {
        let mut res = crack(&cipherbytes, *keylen, &baseline_freqs);
        res.confidence *= keylen_confidence;
        crack_results.push(res);
    }

    // SPELL CHECKING
    for crack in &crack_results {
        spell_checked.push(spellcheck(crack, &bytes_dict));
    }

    let best_after_spellcheck = best_crack(&spell_checked);

    // return the plaintext guess
    return bytes_to_str(&best_after_spellcheck.plaintext);
}
