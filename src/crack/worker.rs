use crate::ciphers::schedulers::RandomScheduler;
use crate::ciphers::{Cipher, Encryptor};
use crate::crack::{best_crack, crack, guesses, spellcheck, Frequencies};
use crate::dict::{BytesDictionary, Dictionary};
use crate::gen::Generator;
use crate::rng::{FromRng, Rng};
use crate::utils::*;

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

pub struct CrackWorker {
    // recv RandomSchedulers
    schedulers: Receiver<RandomScheduler>,
    // send back the RandomScheduler, keylen, and success
    results: Sender<(u8, u8, RandomScheduler, usize, f32)>,
}

pub type WorkerComms = (
    Sender<RandomScheduler>,
    Receiver<(u8, u8, RandomScheduler, usize, f32)>,
    Vec<std::thread::JoinHandle<()>>,
);

pub fn spawn_workers(num_workers: usize) -> WorkerComms {
    let (sched_in, sched_out) = bounded(128);
    let (results_in, results_out) = unbounded();
    let mut rng = Rng::default();

    let mut handles = Vec::new();

    for _ in 0..num_workers {
        let worker = CrackWorker {
            schedulers: sched_out.clone(),
            results: results_in.clone(),
        };

        let seed = rng.next();
        let handle = std::thread::spawn(move || worker.crack_loop(seed));
        handles.push(handle);
    }

    (sched_in, results_out, handles)
}

impl CrackWorker {
    pub fn crack_loop(&self, seed: u64) {
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

        'cracking: loop {
            // clear these vectors
            crack_results.clear();
            spell_checked.clear();

            // get the next scheduler to try to crack
            let sched = self.schedulers.recv().unwrap();

            // generate a key
            let key = Key::from_rng(&mut rng);
            let keylen = key.len();

            // compile the encryptor
            let encryptor = Encryptor::new(key, sched, Rng::from_rng(&mut rng));

            // generate plaintext
            let testtype = if *rng.choose(&[true, false]) { 1 } else { 2 };

            let plaintext = match testtype {
                1 => rng.choose(&test1_known_plaintexts).0.clone(),
                2 => gen.generate_words(200),
                _ => unreachable!(),
            };

            // generate ciphertext
            let ciphertext = encryptor.encrypt(&plaintext);
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
                .send((testtype, 2, encryptor.keyschedule, keylen, success))
                .unwrap();
        }
    }
}
