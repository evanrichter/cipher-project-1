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

impl super::KeySchedule for RepeatingKey {
    fn schedule(&self, index: usize, key_length: usize, _: usize) -> usize {
        index % key_length
    }
}
