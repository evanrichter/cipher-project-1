// the goal of this is to reverse the original key until the offset 
// and then continue with the original key
// this leads to variable effective key length to confuse key length guessing
// for example: ABCDEF with offset 2 would turn into FEDCABCDEF
#[derive(Debug)]
pub struct InvertZip {
    offset: usize,
}
use super::KeySchedule;

impl super::KeySchedule for InvertZip {
    fn schedule(&self, index: usize, key_length: usize, _plaintext_length: usize) -> usize {
        //get the index value of the last character for zero based array
        let last_char = key_length - 1;
        
        if index <= last_char {
            if last_char - index > self.offset {
            // before any repetition
            let inverted_index = last_char - index;
            println!("This is the If. index is {}", inverted_index);
            inverted_index
            } else {
                println!("this is the else1. Index is {} and {}",index, index % last_char);
                index % last_char
            }
        } else {
            // after repeated range
            println!("this is the else2. Index is {} and {}",index, index - key_length);
            //println!("eff {}", eff_key_length);
            index - key_length
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let key = b"ABCDEF";
        let effective_key = b"FEDCBAABCDEF";
        let invertzip = InvertZip {
            offset: 0,
        };

        let mut index = 0;
        println!("key len is {}", key.len());
        println!("effective key len is {}",effective_key.len());
        for _ in 0..500 {
            for expected in 0..effective_key.len() {
                let computed = invertzip.schedule(index, key.len(), 1000);
                println!("{}", key[computed]);
                println!("{}", effective_key[expected]);
                assert_eq!(effective_key[expected], key[computed]);
                index += 1;
            }
        }
    }
}