use super::KeySchedule;
use super::RepeatingKey;

/// This scheduler repeats the key, but overwrites with, or inserts, a random char on a repeating
/// basis.
///
/// This scheduler is can be composed with any other scheduler so that other key schedules can mix
/// in random characters without interrupting the normal rotation.
///
/// By default, PeriodicRand implements [`KeySchedule`] composed with a [`RepeatingKey`].
///
/// Example:
///
/// If the key is `ABCDEFG`, and the key schedule is `PeriodicRand { period: 3, start: 1,
/// overwrite: false }`, then the expected output keystream is `A_BCD_EFG_ABC_DEF_GAB_CDE_FG`
/// repeating, where `_` is some random character.
#[derive(Debug)]
pub struct PeriodicRand {
    /// Number of characters between random chars
    pub period: usize,
    /// Index to put the first random char
    pub start: usize,
    /// Whether to insert random, displacing original keystream, or replacing original key chars
    pub overwrite: bool,
}

impl PeriodicRand {
    /// Test if current index should be rand
    fn random_at(&self, index: usize) -> bool {
        index >= self.start && (index - self.start) % self.period == 0
    }

    /// Calculate how many insertions have been done already
    fn insertions_done(&self, index: usize) -> usize {
        let mut num_insertions = index.saturating_sub(self.start) / self.period;

        if index > self.start {
            num_insertions += 1;
        }

        num_insertions
    }
}

impl KeySchedule for PeriodicRand {
    fn schedule(&self, index: usize, key_length: usize, pt_length: usize) -> usize {
        // assume the PeriodicRand is in front of a simple repeating key schedule by default
        let rk = &RepeatingKey;

        (self, rk).schedule(index, key_length, pt_length)
    }
}

// This impl allows us to chain a PeriodicRand in front of anything that implements KeySchedule.
//
// For example, to do keyscheduling with the Aab scheduler normally, but every 7th character insert
// a random:
//
// ```
// let sched = (PeriodicRand { period: 7, start: 7, overwrite: false }, Aab { .. });
// sched.schedule(0, 12, 500);
// ```
impl<K: KeySchedule> KeySchedule for (&PeriodicRand, &K) {
    fn schedule(&self, mut index: usize, key_length: usize, plaintext_length: usize) -> usize {
        let prand = &self.0;
        let other = &self.1;

        // determine if we should insert/overwrite a random char now
        if prand.random_at(index) {
            return usize::MAX;
        }

        // fix overall index in case we aren't simply overwriting
        if !prand.overwrite {
            index -= prand.insertions_done(index);
        }

        // return whatever the other scheduler does
        other.schedule(index, key_length, plaintext_length)
    }
}

impl crate::rng::FromRng for PeriodicRand {
    fn from_rng(rng: &mut crate::rng::Rng) -> Self {
        Self {
            // make the period at least 32 so we have a chance at recovering plaintext
            period: 32 + rng.next() as usize % 32,
            // let start be anything up to 32
            start: rng.next() as usize % 32,
            // overwrite vs. insert can be random
            overwrite: rng.next() & 1 == 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periodic() {
        let key = b"ABCDEFG";
        let sched = PeriodicRand {
            period: 4,
            start: 1,
            overwrite: false,
        };

        const RAND: usize = usize::MAX;
        #[rustfmt::skip]
        let effective_key_indices = [0, RAND,
                               1, 2, 3, RAND,
                               4, 5, 6, RAND,
                               0, 1, 2, RAND,
                               3, 4, 5, RAND,
                               6, 0, 1, RAND,
                               2, 3, 4, RAND,
                               5, 6];

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

    #[test]
    fn chained_with_aab() {
        use crate::ciphers::schedulers::Aab;

        let key = b"aBCDefg";
        let effective_key = b"aBCD_\
                            BCDBCD_\
                            efgaBC_\
                            DBCDBC_\
                            DefgaB_\
                            CDBCDB_\
                            CDefga_\
                            BCDBCD_\
                            BCDefg_\
                            aBCDBC_\
                            DBCDef_\
                            gaBCDB_\
                            CDBCDe_\
                            fg";
        let aab = Aab {
            num_chars: 3,
            num_reps: 2,
            offset: 1,
        };
        let rand = PeriodicRand {
            period: 7,
            start: 4,
            overwrite: false,
        };

        let sched = (&rand, &aab);

        let mut index = 0;
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = sched.schedule(index, key.len(), 1000);
                if computed == usize::MAX {
                    assert_eq!(effective_key[expected], b'_');
                } else {
                    assert_eq!(effective_key[expected], key[computed]);
                }
                index += 1;
            }
        }
    }
}
