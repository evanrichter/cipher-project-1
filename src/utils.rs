pub trait CharToNum {
    fn to_num(self) -> u8;
}

impl CharToNum for char {
    fn to_num(self) -> u8 {
        // assert that the character is within our defined set ('a-z<space>')
        // note: because of #[cfg(test)], this check is only done during tests!
        #[cfg(test)]
        assert!(self == ' ' || self >= 'a' && self <= 'z');

        match self {
            ' ' => 26,
            c => c as u8 - 'a' as u8,
        }
    }
}

pub trait NumToChar {
    fn to_char(self) -> char;
}

impl NumToChar for u8 {
    fn to_char(self) -> char {
        #[cfg(test)]
        assert!(self <= 26);

        match self {
            26 => ' ',
            n => ('a' as u8 + n) as char,
        }
    }
}
