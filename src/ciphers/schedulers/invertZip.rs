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
        let eff_key_length = key_length + self.offset;
        //let eff_index = index % key_length;
        //let inverted_index = last_char - eff_index;
        
        println!("Index is {}", index);
        //println!("Inverted Index is {}", inverted_index);
        //println!("eff_index is {}", eff_index);
        println!("watch: {}", index % eff_key_length);


        if index % eff_key_length < self.offset {
            println!("here1");
            let inverted_index = last_char - (index % eff_key_length);
            inverted_index
        } else {
            println!("here2");
            let adj_index = (index % eff_key_length) - self.offset;
            adj_index
        }



/*
        if self.offset > index % eff_key_length {
            if eff_index >= self.offset {
                println!("here1");
                inverted_index + self.offset
            } else {
                println!("here2");
                inverted_index
            }
        
        } else {
            // after repeated range
            //println!("this is the else2. Index is {} and {}",index, eff_index);
            //println!("eff {}", eff_key_length);
            if eff_index >= self.offset {
                println!("here3");
                eff_index - self.offset
            } else {
                println!("here4");
                eff_index + (self.offset % last_char)
            }
        }
    */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let key = b"ABCDEF";
        let effective_key = b"ABCDEF";
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
    #[test]
    fn with_offset() {
        let key = b"ABCDEF";
        let effective_key = b"FEDABCDEF";
        let invertzip = InvertZip {
            offset: 3,
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