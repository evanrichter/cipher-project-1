//! Definition of [`KeySchedule`] and various implementations of key scheduling.

mod aab;
mod lengthmod;
mod offsetreverse;
mod periodicrand;
mod repeatingkey;

pub use aab::Aab;
pub use lengthmod::LengthMod;
pub use offsetreverse::OffsetReverse;
pub use periodicrand::PeriodicRand;
pub use repeatingkey::RepeatingKey;

use crate::rng::FromRng;

/// Trait for implementing key scheduling.
pub trait KeySchedule {
    /// Returns the index of the key to use when shifting plaintext into ciphertext.
    ///
    /// In the project description, this process is described as: each ciphertext symbol `c[i]` is the
    /// shift of the plaintext symbol `m[i]` by a number of position equal to one of the key symbols,
    /// which symbol being chosen according to an _undisclosed, deterministic, and not key-based_,
    /// scheduling algorithm that is a function of `i`, `t` and `L`, where:
    ///   * `i` is the index being output to ciphertext
    ///   * `t` is the key length
    ///   * `L` is the length of the plaintext
    fn schedule(&self, index: usize, key_length: usize, plaintext_length: usize) -> NextKey;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NextKey {
    KeyIndex(usize),
    Rand,
}

#[cfg(test)]
impl NextKey {
    /// Gets the key index. Panics if the next key is actually NextKey::Rand. Used for testing only
    /// at this point.
    pub fn index_or_panic(&self) -> usize {
        match self {
            NextKey::KeyIndex(i) => *i,
            NextKey::Rand => panic!("NextKey::Rand does not hold a valid key index"),
        }
    }
}

/// Base scheduler type that exists to randomly generate many kinds of schedulers
#[derive(Debug, Clone, Copy)]
pub enum RandomBaseScheduler {
    Aab(Aab),
    LengthMod(LengthMod),
    OffsetReverse(OffsetReverse),
    RepeatingKey(RepeatingKey),
}

impl FromRng for RandomBaseScheduler {
    fn from_rng(rng: &mut crate::Rng) -> Self {
        match rng.choose(&[1, 2, 3, 4]) {
            1 => Self::Aab(Aab::from_rng(rng)),
            2 => Self::LengthMod(LengthMod),
            3 => Self::OffsetReverse(OffsetReverse::from_rng(rng)),
            4 => Self::RepeatingKey(RepeatingKey),
            _ => unreachable!(),
        }
    }
}

impl KeySchedule for RandomBaseScheduler {
    fn schedule(&self, i: usize, k: usize, p: usize) -> NextKey {
        match self {
            Self::Aab(s) => s.schedule(i, k, p),
            Self::LengthMod(s) => s.schedule(i, k, p),
            Self::OffsetReverse(s) => s.schedule(i, k, p),
            Self::RepeatingKey(s) => s.schedule(i, k, p),
        }
    }
}

/// Overarching scheduler type that exists to randomly generate many kinds of schedulers. At the
/// highest level, there are multiple levels of PeriodicRand, and at the base, any one of the
/// normal schedulers: Aab, LengthMod, OffsetReverse, and RepeatingKey
#[derive(Debug, Clone, Copy)]
pub enum RandomScheduler {
    /// No PeriodicRand layer
    Zero(RandomBaseScheduler),
    /// One PeriodicRand layer
    One(RandomBaseScheduler, PeriodicRand),
    /// Two PeriodicRand layers
    Two(RandomBaseScheduler, PeriodicRand, PeriodicRand),
    /// Three PeriodicRand layers
    Three(
        RandomBaseScheduler,
        PeriodicRand,
        PeriodicRand,
        PeriodicRand,
    ),
}

impl FromRng for RandomScheduler {
    fn from_rng(rng: &mut crate::Rng) -> Self {
        //match rng.choose(&[0, 0, 1, 1, 1, 2, 2, 2, 3]) {
        match rng.choose(&[0]) {
            0 => Self::Zero(RandomBaseScheduler::from_rng(rng)),
            1 => Self::One(
                RandomBaseScheduler::from_rng(rng),
                PeriodicRand::from_rng(rng),
            ),
            2 => {
                // make period less frequent
                let mut pr1 = PeriodicRand::from_rng(rng);
                let mut pr2 = PeriodicRand::from_rng(rng);
                pr1.period += 8;
                pr2.period += 8;
                Self::Two(RandomBaseScheduler::from_rng(rng), pr1, pr2)
            }
            3 => {
                // make period less frequent
                let mut pr1 = PeriodicRand::from_rng(rng);
                let mut pr2 = PeriodicRand::from_rng(rng);
                let mut pr3 = PeriodicRand::from_rng(rng);
                pr1.period += 16;
                pr2.period += 16;
                pr3.period += 16;
                Self::Three(RandomBaseScheduler::from_rng(rng), pr1, pr2, pr3)
            }
            _ => unreachable!(),
        }
    }
}

impl KeySchedule for RandomScheduler {
    fn schedule(&self, idx: usize, k_len: usize, p_len: usize) -> NextKey {
        match self {
            Self::Zero(s) => s.schedule(idx, k_len, p_len),
            Self::One(s, a) => (a, s).schedule(idx, k_len, p_len),
            Self::Two(s, a, b) => (a, &(b, s)).schedule(idx, k_len, p_len),
            Self::Three(s, a, b, c) => (a, &(b, &(c, s))).schedule(idx, k_len, p_len),
        }
    }
}
