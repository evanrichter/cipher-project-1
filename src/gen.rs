use crate::dict::Dictionary;

/// A deterministic plaintext generator. The purpose is to be able to quickly generate known
/// plaintexts so that we can encipher them, and then attempt to crack the ciphertext. Since we
/// generated the plaintext ourself, we can simply compare our cracking results to verify.
#[derive(Clone, Debug)]
pub struct Generator<'d> {
    dictionary: &'d Dictionary,
    rng: Rng,
}

/// This is RomuDuo from www.romu-random.org/code.c
///
/// It generates u64 and is fast, not cryptographically secure, but that's not needed to just
/// generate random plaintexts.
#[derive(Clone, Debug)]
struct Rng {
    x: u64,
    y: u64,
}

impl Default for Rng {
    fn default() -> Self {
        // chosen by fair dice roll
        Self {
            x: 0x54d3a3130133750b,
            y: 0x3e69b0ed931eb512,
        }
    }
}

impl Rng {
    // the reason this doesn't look exactly like the C implementation of RomuDuo is because Rust
    // will panic (safely halt) if any arithmetic overflows in a debug build, such as during `cargo
    // test`. "Release" builds wrap integers silently. We call `u64.wrapping_[mul|add|sub]` to
    // indicate we always intend for this wrapping behavior.
    pub fn next(&mut self) -> u64 {
        let xp = self.x;
        self.x = self.y.wrapping_mul(15241094284759029579);
        self.y = self
            .y
            .rotate_left(36)
            .wrapping_add(self.y.rotate_left(15))
            .wrapping_sub(xp);
        return xp;
    }
}

impl<'d> Generator<'d> {
    /// Instantiate a generator that generates messages using the given [`Dictionary`] as a
    /// wordbank.
    pub fn with_dict(dictionary: &'d Dictionary) -> Self {
        Self {
            rng: Rng::default(),
            dictionary,
        }
    }

    /// Pick `num_words` number of words from the wordbank, join them together with a single space,
    /// then return as a String.
    pub fn generate_words(&mut self, num_words: usize) -> String {
        // create a vector with a big enough allocation to hold `num_words` amount of &str
        let mut sentence = Vec::with_capacity(num_words);

        for _ in 0..num_words {
            // generate a random but valid index into the dictionary
            let index = self.rng.next() as usize % self.dictionary.len();

            // push the &str (pointer to the String + length) into the vector
            sentence.push(self.dictionary.words[index].as_str());
        }

        // join up all those &strs into a space separated String
        sentence.join(" ")
    }
}
