// the goal of this is to reverse the original key until the offset and then continue with the
// original key this leads to variable effective key length to confuse key length guessing
//
// For Example: ABCDEF with offset 2 would turn into FEABCDEF
#[derive(Debug)]
pub struct OffsetReverse {
    offset: usize,
}

use super::KeySchedule;

impl KeySchedule for OffsetReverse {
    fn schedule(&self, index: usize, key_length: usize, _plaintext_length: usize) -> usize {
        //get the index value of the last character for zero based array
        let last_char = key_length - 1;
        let eff_key_length = key_length + self.offset;

        //Before the offset
        if index % eff_key_length < self.offset {
            //calculate the inverted index (index starting from the last character)
            let inverted_index = last_char - (index % eff_key_length);
            inverted_index
        } else {
            //calculate the index adjusting for any previous offset
            let adj_index = (index % eff_key_length) - self.offset;
            adj_index
        }
    }
}

impl crate::rng::FromRng for OffsetReverse {
    fn from_rng(rng: &mut crate::Rng) -> Self {
        Self {
            offset: rng.next() as usize % 17,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let key = b"ABCDEF";
        let effective_key = b"ABCDEF";
        let offsetreverse = OffsetReverse { offset: 0 };

        let mut index = 0;
        println!("key len is {}", key.len());
        println!("effective key len is {}", effective_key.len());
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = offsetreverse.schedule(index, key.len(), 1000);
                println!("{}", key[computed]);
                println!("{}", effective_key[expected]);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
    #[test]
    fn with_offset() {
        let key = b"ABCDEF";
        let effective_key = b"FEDABCDEF";
        let offsetreverse = OffsetReverse { offset: 3 };

        let mut index = 0;
        println!("key len is {}", key.len());
        println!("effective key len is {}", effective_key.len());
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = offsetreverse.schedule(index, key.len(), 1000);
                println!("{}", key[computed]);
                println!("{}", effective_key[expected]);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
    #[test]
    fn full_reverse() {
        let key = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let effective_key = b"ZYXWVUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let offsetreverse = OffsetReverse { offset: 26 };

        let mut index = 0;
        println!("key len is {}", key.len());
        println!("effective key len is {}", effective_key.len());
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = offsetreverse.schedule(index, key.len(), 1000);
                println!("{}", key[computed]);
                println!("{}", effective_key[expected]);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
}
