//! Module for utilities used throughout the cracking tool.

/// The alphabet in the message space
pub const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz ";

/// Extension trait for `char` to be converted to a `i8` according to the encoding scheme
/// 0 => 'a', 1 => 'b', 2 => 'c', ..., 25 => 'z', 26 => ' '. Only the lowercase characters 'a'
/// through 'z' and space ' ' are supported.  converted.
pub trait CharToNum {
    fn to_num(self) -> i8;
}

impl CharToNum for char {
    fn to_num(self) -> i8 {
        // Assert that the character is within our defined set ('a-z<space>') for debug builds.
        // This is not asserted when built with `cargo build --release`.
        debug_assert!(self == ' ' || self >= 'a' && self <= 'z');

        match self {
            ' ' => 26,
            c => c as i8 - 'a' as i8,
        }
    }
}

/// Extension trait for integer types to be converted to a `char` according to the encoding scheme
/// 0 => 'a', 1 => 'b', 2 => 'c', ..., 25 => 'z', 26 => ' '. All other numbers are invalid to be
/// converted.
pub trait NumToChar {
    fn to_char(self) -> char;
}

impl NumToChar for i8 {
    fn to_char(self) -> char {
        const ALPHALEN: i8 = ALPHABET.len() as i8;

        // reduce self to positive integer within ALPHALEN
        let num = self.rem_euclid(ALPHALEN);

        match num {
            26 => ' ',
            n => ('a' as i8 + n) as u8 as char,
        }
    }
}

/// An extension trait to shift by some amount, using modulo to wrap around if needed.
pub trait Shift {
    fn shift(self, amount: i8) -> Self;
}

impl Shift for char {
    fn shift(self, amount: i8) -> Self {
        const ALPHALEN: i8 = ALPHABET.len() as i8;

        // wrap the shift amount to within one alphabet length
        let amount = amount.rem_euclid(ALPHALEN);

        // get numerical value of this char
        let num = self.to_num();

        // add the shift amount, and return as char
        (num + amount).to_char()
    }
}

impl Shift for i8 {
    fn shift(self, amount: i8) -> Self {
        const ALPHALEN: i8 = ALPHABET.len() as i8;

        // wrap the shift amount to within one alphabet length
        let amount = amount.rem_euclid(ALPHALEN);

        // add the shift amount, and mod if needed
        (self + amount).rem_euclid(ALPHALEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shiftchar() {
        // positive shift
        assert_eq!('a'.shift(13), 'n');
        assert_eq!('a'.shift(13 + 27), 'n');
        assert_eq!('a'.shift(13 + 27 * 2), 'n');
        assert_eq!('a'.shift(13 + 27 * 3), 'n');
        assert_eq!('a'.shift(13 + 27 * 4), 'n');

        // negative shift
        assert_eq!('a'.shift(-14), 'n');
        assert_eq!('a'.shift(-14 - 27), 'n');
        assert_eq!('a'.shift(-14 - 27 * 2), 'n');
        assert_eq!('a'.shift(-14 - 27 * 3), 'n');
        assert_eq!('a'.shift(-14 - 27 * 4), 'n');
        assert_eq!('a'.shift(-14 - 27 * 4), 'n');
    }
}

/// The key type defines what format various functions expect the key to be in.
///
/// A [`Vec`] of `i8` representing shift amounts that may be positive or negative.
pub type Key = Vec<i8>;

/// Normalizes a key with arbitrary shift amounts the smallest positive shift amounts.
pub fn reduce_key(key: &mut Key) {
    for k in key.iter_mut() {
        *k = k.to_char().to_num();
    }
}

/// Translate an entire &str to a Vec of bytes to more easily perform math.
#[allow(dead_code)]
pub fn str_to_bytes(s: &str) -> Vec<u8> {
    s.chars().map(|c| c.to_num() as u8).collect()
}

/// Translate a slice of bytes back to a &str for presentation. For example, printing the recovered
/// plaintext as a String.
#[allow(dead_code)]
pub fn bytes_to_str(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| (b as i8).to_char()).collect()
}
