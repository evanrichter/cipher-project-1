/// This key scheduler simply takes the key, and cycles through it, start to finish.
///
/// Example with key `HEADCRAB` and plaintext: `RISE AND SHINE MISTER FREEMAN RISE AND SHINE`:
///
/// ```
///  Plaintext:     RISE AND SHINE MISTER FREEMAN RISE AND SHINE
/// Shifted by:     HEADCRABHEADCRABHEADCRABHEADCRABHEADCRABHEAD
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RepeatingKey;

use super::{KeySchedule, NextKey};

impl KeySchedule for RepeatingKey {
    fn schedule(&self, index: usize, key_length: usize, _: usize) -> NextKey {
        NextKey::KeyIndex(index % key_length)
    }
}
