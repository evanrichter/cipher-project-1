// the goal of this is to "zipper" the original key into an effective key
// for example: ABCDEF would turn into AFBECD
#[derive(Debug)]
pub struct InvertZip {
    offset: usize,
}
use super::KeySchedule;

impl super::KeySchedule for InvertZip {
    fn schedule(&self, index: usize, key_length: usize, _plaintext_length: usize) -> usize {
        let last_char = key_length;
        let counter = 0;
        if index < self.offset {
            // before the zipper starts
            index
        } else if index >= self.offset && ((index-offset) % 2 == 0) {
            // next character in key
            index
        } else if index >= self.offset && ((index - offset) % 2 != 0) {
            // next last character in key
            let inverted_index = last_char - index;
            inverted_index
        } else {
            index
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let key = b"ABCDEFG";
        let effective_key = b"AGBFCED";
        let invertzip = InvertZip {
            offset: 0,
        };

        let mut index = 0;
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = invertzip.schedule(index, key.len(), 1000);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
}