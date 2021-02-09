/// A dictionary will hold an alphabetized wordlist. Each word only consists of lowercase ASCII
/// alphabetic characters.
#[derive(Clone, Debug)]
pub struct Dictionary {
    pub words: Vec<String>,
}

impl Dictionary {
    /// Create a dictionary from a single string that is whitespace  separated. This function will
    /// work for both:
    ///  * the project specified dictionary input (a file with space separated words)
    ///  * the common file format for words files (newline separated words)
    ///
    ///  This function also rejects any word that contains non alphabetic ascii characters,
    ///  printing to stderr for each word it tosses out.
    pub fn from_str(source: String) -> Self {
        let mut words: Vec<String> = source
            // trim off starting and trailing whitespace
            .trim()
            // split by any type of ascii whitespace
            .split_ascii_whitespace()
            // make sure the word is only a-zA-Z
            .filter(|word| {
                let alphabetic = word.chars().all(|chr| chr.is_alphabetic());
                if !alphabetic {
                    eprintln!("word \"{}\" is non-alphabetic", word);
                }
                alphabetic
            })
            // lowercase the word
            .map(|word| word.to_ascii_lowercase())
            // call next on the iterator, building up a single Vec
            .collect();

        // sort the words alphabetically
        words.sort();

        // return the dictionary
        Self { words }
    }

    /// Return how many words are in the dictionary
    pub fn len(&self) -> usize {
        self.words.len()
    }
}
