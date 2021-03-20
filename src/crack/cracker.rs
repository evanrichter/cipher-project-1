use crate::ciphers::schedulers::RandomScheduler;
use crate::ciphers::{Cipher, Encryptor};
use crate::crack::{best_crack, crack, guesses, spellcheck, Frequencies};
use crate::dict::{BytesDictionary, Dictionary};
use crate::gen::Generator;
use crate::rng::{FromRng, Rng};
use crate::utils::*;


pub struct CrackableCipher {
    // received ciphertext
    schedulers: Receiver<String>,
    // resulting plaintext
    plaintext: Sender<(Vec<u8>, f32)>,
}

impl CrackableCipher {
    pub fn crack_single_ciphertext(&self){
        // SETUP
        let mut words = include_str!("../../words/default.txt").to_string();
        let dict = Dictionary::from_string(&mut words);
        let bytes_dict = BytesDictionary::from_dict(&dict);
        let baseline_freqs = Frequencies::from_dict(&dict);

        // Get strings for Test 1
        let test1_str = include_str!("../../words/test1_plaintext.txt");
        let test1_known_plaintexts: Vec<(String, Frequencies)> = test1_str
            .lines()
            .map(|s| {
                let string = s.to_string();
                let freqs = Frequencies::from_str(s);
                (string, freqs)
            })
            .collect();

        let mut gen = Generator::with_dict(&dict);
        let mut rng = Rng::with_seed(seed, seed);

        let mut keylen_guesses = Vec::new();
        let mut crack_results = Vec::new();
        let mut spell_checked = Vec::new();

        // clear these vectors
        crack_results.clear();
        spell_checked.clear();

        // get the next scheduler to try to crack
        //let sched = self.schedulers.recv().unwrap();

        // generate a key
        //let key = Key::from_rng(&mut rng);
        //let keylen = key.len();

        // compile the encryptor
        //let encryptor = Encryptor::new(key, sched, Rng::from_rng(&mut rng));

        // generate plaintext
        //let testtype = if *rng.choose(&[true, false]) { 1 } else { 2 };

        //let plaintext = match testtype {
        //    1 => rng.choose(&test1_known_plaintexts).0.clone(),
        //    2 => gen.generate_words(200),
        //    _ => unreachable!(),
        //};


        // generate ciphertext
        let ciphertext = self.ciphertext;
        let cipherbytes = str_to_bytes(&ciphertext);

        // KEYLENGTH GUESSING
        guesses(&cipherbytes, &mut keylen_guesses);

        // ===============   TEST 1   ===================== //

        let mut best_test1_score = f32::MAX;

        for (known_pt, freqs) in test1_known_plaintexts.iter() {
            let mut best_score = f32::MAX;

            for crack in (3..120_usize).map(|keylen| crack(&cipherbytes, keylen, &freqs)) {
                let crackstr = bytes_to_str(&crack.plaintext);
                let score =
                    strsim::levenshtein(&crackstr, &known_pt) as f32 / plaintext.len() as f32;

                // update the best score for this plaintext
                if score < best_score {
                    best_score = score;
                }
            }

            if best_score < best_test1_score {
                best_test1_score = best_score;
            }
        }

        if best_test1_score < 0.8 {
            // it was probably test1, send back results
            self.results
                .send((testtype, 1, encryptor.keyschedule, keylen, best_test1_score))
                .unwrap();

            // continue main cracking loop
            continue 'cracking;
        }

        // ===============   TEST 2   ===================== //

        // CRACKING SLICES
        for (keylen, keylen_confidence) in keylen_guesses.iter() {
            let mut res = crack(&cipherbytes, *keylen, &baseline_freqs);
            res.confidence *= keylen_confidence;
            crack_results.push(res);
        }

        // SPELL CHECKING
        for crack in &crack_results {
            spell_checked.push(spellcheck(crack, &bytes_dict));
        }

        let best_after_spellcheck = best_crack(&spell_checked);

        let success =
            strsim::levenshtein(&bytes_to_str(&best_after_spellcheck.plaintext), &plaintext)
                as f32
                / plaintext.len() as f32;

        // send back the results
        self.results
            .send((spell_checked, success))
            .unwrap();
    }
}