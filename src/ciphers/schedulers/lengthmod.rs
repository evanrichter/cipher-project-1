//This scheduler mods the plaintext length to produce a the index. THe final result is also a function of index and key_lenth in order to inject a certain amount of randomness and because we didn't have an example where all 3 variables were used. 

#[derive(Debug)]
pub struct Lengthmod{}   

use super::KeySchedule;

impl KeySchedule for Lengthmod {
    fn schedule(&self, index: usize, key_length: usize, _plaintext_length: usize) -> usize {
         if _plaintext_length< (index*key_length){
            let mut returnInd = _plaintext_length % key_length;
            returnInd
        }else{
            let mut returnInd =(_plaintext_length*index) % key_length;
            returnInd
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //tests the first branch option, if the resulting index is 0, its working correctly
    fn test_lengthmod(){
        let mut res = lengthmod.schedule(12, 50, 500);
        assert_eq!(res, 0);
    }

    #[test]
    //tests the second branch option, if the resulting index is 1, its working correctly
    fn test_lengthmod(){
        let mut res = lengthmod.schedule(1, 50, 501);
        assert_eq!(res, 1);
    }
