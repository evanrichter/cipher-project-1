//This scheduler mods the plaintext length to produce a the index. THe final result is also a function of index and key_lenth in order to inject a certain amount of randomness and because we didn't have an example where all 3 variables were used.

#[derive(Debug, Clone, Copy)]
pub struct LengthMod;

use super::{KeySchedule, NextKey};

impl KeySchedule for LengthMod {
    fn schedule(&self, index: usize, key_length: usize, plaintext_length: usize) -> NextKey {
        let next = if plaintext_length < (index * key_length) {
            plaintext_length % key_length
        } else {
            (plaintext_length * index) % key_length
        };

        NextKey::KeyIndex(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // tests the first branch option, if the resulting index is 0, its working correctly
    fn first() {
        let sched = LengthMod {};
        let res = sched.schedule(12, 50, 500).index_or_panic();
        assert_eq!(res, 0);
    }

    #[test]
    // tests the second branch option, if the resulting index is 1, its working correctly
    fn second() {
        let sched = LengthMod {};
        let res = sched.schedule(1, 50, 501).index_or_panic();
        assert_eq!(res, 1);
    }
}
