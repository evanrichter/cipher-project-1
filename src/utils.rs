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

/// An extension trait to shift `char` by some amount, using modulo to wrap around if needed.
pub trait ShiftChar {
    fn shift(self, amount: i8) -> Self;
}

impl ShiftChar for char {
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

/// Normalizes a key with arbitrary shift amounts the smallest positive shift amounts.
pub fn reduce_key(key: &mut Vec<i8>) {
    for k in key.iter_mut() {
        *k = k.to_char().to_num();
    }
}
