use crate::dict::Dictionary;

/// A deterministic plaintext generator. The purpose is to be able to quickly generate known
/// plaintexts so that we can encipher them, and then attempt to crack the ciphertext. Since we
/// generated the plaintext ourself, we can simply compare our cracking results to verify.
#[derive(Clone, Debug)]
pub struct Generator<'d> {
    dictionary: &'d Dictionary,
    rng: Rng,
}

/// This is [RomuDuo]
///
/// It generates u64 and is fast, not cryptographically secure, but that's not needed to just
/// generate random plaintexts.
///
/// [RomuDuo]: https://www.romu-random.org/code.c
#[derive(Clone, Debug)]
pub struct Rng {
    x: u64,
    y: u64,
}

impl Default for Rng {
    fn default() -> Self {
        // chosen by fair dice roll (actually python.random)
        Self {
            x: 0x54d3a3130133750b,
            y: 0x3e69b0ed931eb512,
        }
    }
}

impl Rng {
    /// Returns the next random `u64` from the generator, and updates the internal state of the
    /// generator.
    ///
    /// Note that the method takes `&mut self` (a mutable reference to the Rng struct). This is
    /// because it must update the internal fields of the generator according to the PRNG
    /// algorithm.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// let mut rng = Rng::default();
    /// println!("random u64: {}", rng.next());
    ///
    /// // truncate a u64 to a u32
    /// println!("random u32: {}", rng.next() as u32);
    ///
    /// // truncate a u64 to a single byte
    /// println!("random u8: {}", rng.next() as u8);
    /// ```
    pub fn next(&mut self) -> u64 {
        // the reason this doesn't look exactly like the C implementation of RomuDuo is because
        // Rust will panic (safely halt) if any arithmetic overflows in a debug build, such as
        // during `cargo test`. "Release" builds wrap integers silently. We call
        // `u64.wrapping_[mul|add|sub]` to indicate we always intend for this wrapping behavior.
        let xp = self.x;
        self.x = self.y.wrapping_mul(15241094284759029579);
        self.y = self
            .y
            .rotate_left(36)
            .wrapping_add(self.y.rotate_left(15))
            .wrapping_sub(xp);
        return xp;
    }

    /// Initialize the Rng with set values, or seeds. RomuDuo authors say that any non-zero value
    /// should be fine, but seeds with very few bits set will produce low quality random values to
    /// start.
    ///
    /// To prevent problems with this, `with_seed` asserts that neither starting state is zero, and
    /// runs 100 iterations of [`next`][`Rng::next`] before returning the resulting Rng.
    pub fn with_seed(x: u64, y: u64) -> Self {
        assert!(x != 0 && y != 0, "seed values should not be zero!");
        let mut rng = Self { x, y };
        for _ in 0..100 {
            rng.next();
        }
        rng
    }

    /// Choose an item from a slice of items.
    ///
    /// ```
    /// let mut rng = Rng::default();
    /// let choices = [1, 1, 1, 2, 3, 4, 4, 8];
    /// 
    /// println!("first choice is {}", rng.choose(&choices));
    /// println!("first choice is {}", rng.choose(&choices));
    /// ```
    pub fn choose<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
        // generate a random but valid index
        let index = self.next() as usize % choices.len();
        &choices[index]
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
            // choose a word at random
            let word = self.rng.choose(&self.dictionary.words);

            // push the &str (pointer to the String + length) into the vector
            sentence.push(word.as_str());
        }

        // join up all those &strs into a space separated String
        sentence.join(" ")
    }
}

// Tests for the Generator type. These get run with `cargo test`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_words() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        let mut g = Generator::with_dict(&d);
        assert_eq!("jkl", g.generate_words(1));
        assert_eq!("ghi", g.generate_words(1));
        assert_eq!("ghi", g.generate_words(1));
        assert_eq!("abc", g.generate_words(1));
        assert_eq!("abc", g.generate_words(1));
        assert_eq!("abc", g.generate_words(1));
        assert_eq!("def", g.generate_words(1));
    }

    #[test]
    fn generate_sentence() {
        let s = String::from("abc def ghi jkl");
        let d = Dictionary::from_string(s);

        let mut g = Generator::with_dict(&d);
        assert_eq!("jkl ghi ghi abc abc abc def", g.generate_words(7));
    }

    #[test]
    #[should_panic]
    fn bad_seed_x() {
        let _ = Rng::with_seed(0, 3);
    }

    #[test]
    #[should_panic]
    fn bad_seed_y() {
        let _ = Rng::with_seed(29, 0);
    }

    #[test]
    fn unique_output_from_different_seeds() {
        let mut a = Rng::with_seed(0x918273498, 0x878787584);
        let mut b = Rng::with_seed(9555, 0x1337_c0de);

        for _ in 0..0x80000 {
            assert_ne!(a.next(), b.next());
        }
    }

    #[test]
    fn choose() {
        let choices = [0,1,2,3,4,5];
        let mut rng = Rng::default();

        // should be able to pick all options within 100 tries
        let mut chosen = vec![false; choices.len()];
        for _ in 0..100 {
            let x = *rng.choose(&choices) as usize;
            chosen[x] = true;
        }
        assert!(chosen.iter().all(|&x| x));

        for _ in 0..10000 {
            let x = *rng.choose(&choices) as usize;
            assert!(x <= 5);
        }
    }
}
