//! Module for correcting nearly perfect plaintext, into a plausible plaintext that actually could
//! have been generated from the source dictionary.

use core::f64;

use strsim::levenshtein;


use super::CrackResult;
use crate::Dictionary;

/// This function exploits the fact that we know the source dictionary (or can guess between a
/// small number of dictionaries), and uses spell checking strategies to fix up any incorrectly
/// guessed shift values from the previous step.



#[allow(dead_code)]
pub fn spellcheck(cracked: &CrackResult, dict: &Dictionary) -> CrackResult {

    // do stuff here to create a real spell checked result


    
    let mut correcttext = cracked.plaintext.clone();
    let iteratortext = cracked.plaintext.clone();
    

    let mut newconfidence: f64 = 0.0;


   for word in iteratortext.split_whitespace() //split_whitespace chunks the string out into words and returns an iterator
   {

    let dictMatch = dict.best_levenshtein(word); //finds the closest word match between the dict and our plaintext result

    if dictMatch.1 == 0 //if its an exact match, we do nothing
    {
        continue; 
    }
    else //if it isn't, we make a change
    {
        correcttext= correcttext.replace(word, dictMatch.0 );
        newconfidence = newconfidence+ 1.0;

    }
}

let numberofdif = levenshtein( &cracked.plaintext, &correcttext) as f64;//finds the number of changes made between the corrected and origional

println!("divisor {}\n", cracked.plaintext.len());
let newconfidence: f64 = numberofdif/cracked.plaintext.len() as f64; //takes the number of levenstein differences over hte length. THis is the ratio between changes made and the length of the text. the higher the value the lower the confidence
 
let errorcorrect =CrackResult{
    plaintext: correcttext.clone(),
    confidence: newconfidence,
};


    errorcorrect
   
}


  #[allow(dead_code)]
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn testing(){
            let cracked= CrackResult{
                plaintext: "wordss wishes this pig the quics brown fox jumpede over the lazy dog cat lion seal fish canary sf f a fash carp sharks".to_string(),
                confidence: 4000.0
            };

            let targetplaintext=String::from("words wishes that pig the quick brown fox jumped over the lazy dog cat lion seal fish canary sf f a fish carp shark");

            let dict= Dictionary{
                words: ["words", "wards", "wishes", "that", "pig", "the", "quick", "brown", "fox", "jumped", "over", "the", "lazy", "dog", "cat", "lion", "seal", "fish", "canary", "sf", "f", "a", "fosh", "carp", "shark", "pie", "sandle", "counter", "keyboard", "airplane", "fresh", "wishes"].to_vec()
            };
           // cracked.plaintext = "wards wishes this pig the quics brown fox jumpede over the lazy dog cat lion seal fish canary sf f a fash carp sharks".to_string();
            println!("BEFORE TEST plaintext is  {}\n", cracked.plaintext);

           let mut errorcorrect= spellcheck(&cracked, &dict);

            println!("AFTER TEST plaintext is  {}\n", errorcorrect.plaintext);

            assert_eq!(errorcorrect.plaintext,targetplaintext); 

        }}

