//! Module for correcting nearly perfect plaintext, into a plausible plaintext that actually could
//! have been generated from the source dictionary.

use super::CrackResult;
use crate::Dictionary;

/// This function exploits the fact that we know the source dictionary (or can guess between a
/// small number of dictionaries), and uses spell checking strategies to fix up any incorrectly
/// guessed shift values from the previous step.
pub fn spellcheck(cracked: &CrackResult, dict: &Dictionary) -> CrackResult {
    println!("cracked plaintext is currently {}", cracked.plaintext);
    println!("    confidence: {}", cracked.confidence);

    // do stuff here to create a real spell checked result
    let corrected = CrackResult { plaintext: "wow".to_string(), confidence: 4000.0 };

    // a test could assert something like this:
    for word in corrected.plaintext.split_whitespace() {
        assert!(dict.words.contains(&word));
    }

    // return the spell checked result
    corrected
}
