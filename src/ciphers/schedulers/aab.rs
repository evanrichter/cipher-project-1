use super::KeySchedule;

/// This scheduler repeats the first half of the key, then runs through the whole key. The hope is
/// to confuse keylength guessing.
///
/// It is called "AAB" scheduler because if the key is "AB" then this scheduler could produce an
/// effective key of "AAB"
#[derive(Debug)]
pub struct Aab {
    /// Number of characters to repeat in the key
    pub num_chars: usize,
    /// Number of times to repeat the block of characters
    pub num_reps: usize,
    /// Offset into the original key to start the repetition
    pub offset: usize,
}

impl KeySchedule for Aab {
    fn schedule(&self, index: usize, key_length: usize, _plaintext_length: usize) -> usize {
        // effective key length is key_length + number of repeated chars
        let eff_key_length = key_length + self.num_chars * self.num_reps;

        // effective index
        let index = index % eff_key_length;

        if index < self.offset {
            // before any repetition
            index
        } else if index < self.offset + (self.num_reps + 1) * self.num_chars {
            // within the repetition range
            (index - self.offset) % self.num_chars + self.offset
        } else {
            // after repeated range
            index - self.num_chars * self.num_reps
        }
    }
}

impl crate::rng::FromRng for Aab {
    fn from_rng(rng: &mut crate::rng::Rng) -> Self {
        Self {
            num_chars: rng.next() as usize % 32,
            num_reps: rng.next() as usize % 8,
            offset: rng.next() as usize % 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetition() {
        let key = b"ABCdefg";
        let effective_key = b"ABCABCdefg";
        let aab = Aab {
            num_chars: 3,
            num_reps: 1,
            offset: 0,
        };

        let mut index = 0;
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = aab.schedule(index, key.len(), 1000);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }

    #[test]
    fn double_repetition() {
        let key = b"ABCdefg";
        let effective_key = b"ABCABCABCdefg";
        let aab = Aab {
            num_chars: 3,
            num_reps: 2,
            offset: 0,
        };

        let mut index = 0;
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = aab.schedule(index, key.len(), 1000);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }

    #[test]
    fn offset() {
        let key = b"aBCDefg";
        let effective_key = b"aBCDBCDBCDefg";
        let aab = Aab {
            num_chars: 3,
            num_reps: 2,
            offset: 1,
        };

        let mut index = 0;
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = aab.schedule(index, key.len(), 1000);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
}
