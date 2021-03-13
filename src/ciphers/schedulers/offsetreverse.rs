// the goal of this is to reverse the original key until the offset and then continue with the
// original key this leads to variable effective key length to confuse key length guessing
//
// For Example: ABCDEF with offset 2 would turn into FEABCDEF
#[derive(Debug, Clone, Copy)]
pub struct OffsetReverse {
    offset: usize,
}

use super::KeySchedule;

impl KeySchedule for OffsetReverse {
    fn schedule(&self, index: usize, key_length: usize, _plaintext_length: usize) -> usize {
        // fix the offset if it's larger than the key
        let offset = self.offset % (key_length + 1);

        //get the index value of the last character for zero based array
        let eff_key_length = key_length + offset;
        let eff_index = index % eff_key_length;

        //Before the offset
        if eff_index < offset {
            //calculate the inverted index (index starting from the last character)
            eff_key_length - eff_index - offset - 1
        } else {
            //calculate the index adjusting for any previous offset
            eff_index - offset
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
                println!("computed  {}", key[computed] as char);
                println!("should be {}", effective_key[expected] as char);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
}
