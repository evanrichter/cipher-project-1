use super::KeySchedule;

/// This scheduler repeats the key, but overwrites with, or inserts, a random char on a repeating
/// basis.
///
/// This scheduler is can be composed with any other scheduler so that other key schedules can mix
/// in random characters without interrupting the normal rotation.
///
/// Example:
///
/// If the key is `ABCDEFG`, and the key schedule is `PeriodicRand { period: 3, start: 1,
/// overwrite: false }`, then the expected output keystream is `A_BCD_EFG_ABC_DEF_GAB_CDE_FG`
/// repeating, where `_` is some random character.
#[derive(Debug)]
pub struct PeriodicRand {
    /// Number of characters between random chars
    period: usize,
    /// Index to put the first random char
    start: usize,
    /// Whether to insert random, displacing original keystream, or replacing original key chars
    overwrite: bool,
}

impl PeriodicRand {
    #[allow(dead_code)]
    pub fn new(period: usize, start: usize, overwrite: bool) -> Self {
        Self {
            period: period + 1,
            start,
            overwrite,
        }
    }
}

impl KeySchedule for PeriodicRand {
    fn schedule(&self, index: usize, key_length: usize, _: usize) -> usize {
        // determine if we should insert/overwrite a random char now
        let rand_now = index >= self.start && (index - self.start) % self.period == 0;

        if rand_now {
            return usize::MAX;
        }

        if self.overwrite {
            return index % key_length;
        } else {
            // how many insertions have been done already
            let mut num_insertions = index.saturating_sub(self.start) / self.period;

            if index > self.start {
                num_insertions += 1;
            }

            return (index - num_insertions) % key_length;
        }
    }
}

impl crate::rng::FromRng for PeriodicRand {
    fn from_rng(rng: &mut crate::rng::Rng) -> Self {
        Self {
            period: 40 + rng.next() as usize % 32,
            start: rng.next() as usize % 32,
            overwrite: rng.next() & 1 == 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetition() {
        let key = b"ABCDEFG";
        let sched = PeriodicRand::new(3, 1, false);

        const RAND: usize = usize::MAX;
        let effective_key_indices = [
            0, RAND, 1, 2, 3, RAND, 4, 5, 6, RAND, 0, 1, 2, RAND, 3, 4, 5, RAND, 6, 0, 1, RAND, 2,
            3, 4, RAND, 5, 6,
        ];

        let mut index = 0;
        for _ in 0..500 {
            for expected in 0..effective_key_indices.len() {
                let computed = sched.schedule(index, key.len(), 1000);
                assert_eq!(effective_key_indices[expected], computed);
                if computed != RAND {
                    assert_eq!(key[effective_key_indices[expected]], key[computed]);
                }
                index += 1;
            }
        }
    }
}
