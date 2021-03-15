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
    results: Sender<(RandomScheduler, usize, f32)>,
}

pub type WorkerComms = (
    Sender<RandomScheduler>,
    Receiver<(RandomScheduler, usize, f32)>,
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
        let mut words = std::fs::read_to_string("words/default.txt").unwrap();
        let dict = Dictionary::from_string(&mut words);
        let bytes_dict = BytesDictionary::from_dict(&dict);
        let baseline_freqs = Frequencies::from_dict(&dict);

        let mut gen = Generator::with_dict(&dict);
        let mut rng = Rng::with_seed(seed, seed);

        let mut keylen_guesses = Vec::new();
        let mut crack_results = Vec::new();
        let mut spell_checked = Vec::new();

        loop {
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

            // generate plaintext and ciphertext
            let plaintext = gen.generate_words(200);
            let ciphertext = encryptor.encrypt(&plaintext);
            let cipherbytes = str_to_bytes(&ciphertext);

            // KEYLENGTH GUESSING
            guesses(&cipherbytes, &mut keylen_guesses);

            // CRACKING SLICES
            for (keylen, _) in keylen_guesses.iter().take(30) {
                let res = crack(&cipherbytes, *keylen, &baseline_freqs);
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
                .send((encryptor.keyschedule, keylen, success))
                .unwrap();
        }
    }
}
