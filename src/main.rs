// these "mod" statements bring in ciphers/mod.rs, dict.rs, gen.rs, and utils.rs files
mod ciphers;
mod crack;
mod dict;
mod gen;
mod rng;
mod utils;

// these "use" statements bring the structs into scope
pub use ciphers::Encryptor;
pub use dict::{BytesDictionary, Dictionary};
pub use gen::Generator;
pub use rng::{FromRng, Rng};

// this "use" statement brings the traits into scope so we can use their methods
pub use ciphers::{Cipher, KeySchedule};

fn main() -> anyhow::Result<()> {
    use crate::ciphers::schedulers::RandomScheduler;
    use crate::crack::{best_crack, crack, guesses, spellcheck, Frequencies};
    use crate::utils::*;

    //
    // SETUP
    //

    let mut words = std::fs::read_to_string("words/default.txt")?;
    let dict = Dictionary::from_string(&mut words);
    let bytes_dict = BytesDictionary::from_dict(&dict);
    let baseline_freqs = Frequencies::from_dict(&dict);

    let mut gen = Generator::with_dict(&dict);
    let mut rng = Rng::with_seed(0xdeadbeef, 0x2dead4beef);

    let mut crack_results = Vec::new();
    let mut spell_checked = Vec::new();

    loop {
        let sched = RandomScheduler::from_rng(&mut rng);

        for _ in 0..5 {
            let key = Key::from_rng(&mut rng);
            let keylen = key.len();

            let encryptor = Encryptor::new(key, sched, Rng::from_rng(&mut rng));

            let plaintext = gen.generate_words(300);
            let ciphertext = encryptor.encrypt(&plaintext);

            let cipherbytes = str_to_bytes(&ciphertext);

            //
            // KEYLENGTH GUESSING
            //

            let keylen_guesses = guesses(&cipherbytes);

            //
            // CRACKING SLICES
            //

            for (keylen, _) in keylen_guesses.iter().take(30) {
                let res = crack(&cipherbytes, *keylen, &baseline_freqs);
                crack_results.push(res);
            }

            //let best_after_crack = best_crack(&crack_results);

            //
            // SPELL CHECKING
            //

            for crack in &crack_results {
                spell_checked.push(spellcheck(crack, &bytes_dict));
            }

            let best_after_spellcheck = best_crack(&spell_checked);

            if bytes_to_str(&best_after_spellcheck.plaintext) != plaintext {
                /*
                println!("cracked and spell checked result does not match actual plaintext");
                println!("encrypted:\n{}\n", ciphertext);
                println!("plaintext:\n{}\n", plaintext);
                println!(
                "best crack result from known keylength:\n{}\n",
                bytes_to_str(&best_after_crack.plaintext)
                );
                println!(
                "best crack result after spell check:\n{}\n",
                bytes_to_str(&best_after_spellcheck.plaintext)
                );
                */
                println!(
                    "failed!  scheduler: {:?} keylen: {}",
                    encryptor.keyschedule, keylen
                );
            } else {
                println!(
                    "success! scheduler: {:?} keylen: {}",
                    encryptor.keyschedule, keylen
                );
            }

            // clear these vectors for next round
            crack_results.clear();
            spell_checked.clear();
        }
    }
}

#[cfg(test)]
#[test]
fn test_main() {
    main().expect("main threw an error");
}
