/// Guess the keylength based on the technique shown in
/// [cryptopals](https://cryptopals.com/sets/1/challenges/6). It is yet to be tested on these shift
/// based ciphers, but this implementation worked against the linked cryptopals challenge based on
/// multi-byte xor.
#[allow(dead_code)]
pub fn guesses(ciphertext: &[u8], keysizes: &mut Vec<(usize, f64)>) {
    // clear previous keysizes
    keysizes.clear();

    const KEYSIZE_LO: usize = 3;
    const KEYSIZE_HI: usize = 120;

    for keysize in KEYSIZE_LO..KEYSIZE_HI {
        let score = hamming_distance_between_chunks(ciphertext, keysize);
        keysizes.push((keysize, score));
    }

    // figure out y = mx + b
    let xy: Vec<_> = keysizes.iter().map(|(a, b)| (*a as f64, *b)).collect();
    let (xtot, ytot) = xy
        .iter()
        .fold((0.0, 0.0), |(sa, sb), (a, b)| (sa + a, sb + b));
    let (xmean, ymean) = (xtot / xy.len() as f64, ytot / xy.len() as f64);
    let (m, b) = linreg::lin_reg(xy.into_iter(), xmean, ymean).unwrap();

    // undo the y = mx + b and normalize to x again
    for (x, y) in keysizes.iter_mut() {
        *y = ((*y - b) + m * (*x as f64)) / *x as f64;
    }

    // sort by best keysize, lowest first
    keysizes.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
}

/// Take 4 chunks of size `chunksize` and calculate a normalized score of the Hamming distance
/// between each chunk.
pub fn hamming_distance_between_chunks(input: &[u8], chunksize: usize) -> f64 {
    let chunks: Vec<&[u8]> = input.chunks_exact(chunksize).collect();
    let mut distance = 0;
    for ii in 0..chunks.len() {
        for jj in ii..chunks.len() {
            distance += hamming_distance(chunks[ii], chunks[jj]);
        }
    }

    distance as f64 / chunks.len() as f64
}

/// Calculate the bitwise Hamming distance between two `u8` slices
pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len(), "lengths must be equal");

    a.iter()
        .zip(b.iter())
        // XOR leaves a 1 where the bits differ. Then counting the ones in the u8 gives the hamming
        // distance for that one byte
        .map(|(a, b)| (a ^ b).count_ones())
        // add all the single byte hamming distances
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::{Cipher, Encryptor, KeySchedule};
    use crate::rng::FromRng;
    use crate::rng::Rng;

    // import schedulers we need
    use crate::ciphers::schedulers::{PeriodicRand, RepeatingKey};

    fn expected_keylen_rank<K>(keylen: usize, sched: K, expected_keylen: usize)
    where
        K: KeySchedule + core::fmt::Debug,
    {
        let mut rng = Rng::default();

        // build the key
        let mut key = vec![0; keylen];
        for k in key.iter_mut() {
            *k = (rng.next() >> 32) as u8 as i8;
        }

        // generate plaintext
        let mut words = std::fs::read_to_string("words/default.txt").unwrap();
        let dict = crate::dict::Dictionary::from_string(&mut words);
        let mut gen = crate::gen::Generator::with_dict(&dict);
        let plaintext = gen.generate_words(1000);

        // create the encryptor
        let encryptor = Encryptor::new(key, sched, rng);

        // encrypt to ciphertext
        let ciphertext = encryptor.encrypt(&plaintext);

        // get ciphertext into bytes
        let ciphertext = crate::utils::str_to_bytes(&ciphertext);

        // calculate guesses
        let mut keysizes = Vec::new();
        guesses(&ciphertext, &mut keysizes);

        // count how many integer multiples (including exact matches) of the expected keylength are
        // in the top results
        let integer_multiples = keysizes
            .iter()
            .map(|(guess, _score)| guess)
            // only look at the top 5 guesses
            .take(5)
            // filter where the expected keylength is a factor of the guess
            .filter(|&&guess| guess == expected_keylen)
            // count how many are left
            .count();

        assert!(integer_multiples > 0, "keylength not in top 5");
    }

    #[test]
    fn simple_repeating_key() {
        // pick a keylen, and period
        let keylen = 13;

        // configure the scheduler to be repeating key
        let sched = RepeatingKey;

        // since the key is very simply repeating, the expected keylen is the same
        let expected_keylen = keylen;

        expected_keylen_rank(keylen, sched, expected_keylen);
    }

    #[test]
    fn rand_with_overwrite() {
        // pick a keylen, and period
        let keylen = 7;
        let period = 9;

        // configure the random char insertions
        let rand_with_overwrite = PeriodicRand {
            period,
            start: 9,
            overwrite: true,
        };

        // since we are _overwriting_ the key every 9 times, but the underlying key still repeats
        // every 7, I would expect we could crack this ciphertext using a keylength of 7 still.
        let expected_keylen = keylen;

        expected_keylen_rank(keylen, rand_with_overwrite, expected_keylen);
    }

    #[test]
    fn rand_with_insert() {
        // pick a keylen, and period
        let keylen = 7;
        let period = 4;

        // configure the random char insertions
        let inserted_rand = PeriodicRand {
            period,
            start: 6,
            overwrite: false,
        };

        // since we are _inserting_ a random char into the key every 4 times, but the underlying
        // key still repeats every 7, I would expect the effective key (the pattern that actually
        // repeats exactly) to be the Least Common Multiple of 7 and 4, or 28
        let expected_keylen = 28;

        expected_keylen_rank(keylen, inserted_rand, expected_keylen);
    }

    /// stress testing keylength guessing
    #[test]
    #[ignore]
    fn stresstest() {
        let mut rng = Rng::default();

        // plaintext generator
        let mut words = std::fs::read_to_string("words/default.txt").unwrap();
        let dict = crate::dict::Dictionary::from_string(&mut words);
        let mut gen = crate::gen::Generator::with_dict(&dict);

        // reusable Vecs for ciphertext, key, plaintext
        let mut plaintext = String::new();
        let mut ciphertext = String::new();
        let mut key = Vec::new();
        let mut keysizes = Vec::new();

        // total runs to do
        const RUNS: usize = 1000;

        // total number of "failures" where the correct keylength was not in the top 15 results
        let mut failures = 0;

        for _ in 0..RUNS {
            // choose a keylength between 8 and 32
            let keylen = rng.next() % 30 + 8;

            // build the key
            for _ in 0..keylen {
                key.push(rng.next() as u8 as i8);
            }

            // build the plaintext
            gen.generate_words_into(120, &mut plaintext);

            // create the encryptor
            // TODO: generate a random scheduler
            let enc_rng = FromRng::from_rng(&mut rng);
            let encryptor = Encryptor::new(key.clone(), RepeatingKey, enc_rng);

            // encrypt to ciphertext
            encryptor.encrypt_into(&plaintext, &mut ciphertext);
            let ct_bytes = crate::utils::str_to_bytes(&ciphertext);

            // get keylength guesses
            guesses(&ct_bytes, &mut keysizes);

            // count how many integer multiples (including exact matches) of the expected keylength are
            // in the top results
            let guessed = keysizes
                .iter()
                .map(|(guess, _score)| guess)
                // only look at the top 15 guesses
                .take(15)
                // filter where the expected keylength is a factor of the guess
                .any(|&guess| guess == keylen as usize);

            if !guessed {
                failures += 1;
            }

            // clear the buffers but keep allocation around
            ciphertext.clear();
            plaintext.clear();
            key.clear();
        }

        println!("successes: {}", RUNS - failures);
        println!("failures: {}", failures);
        assert!(
            (failures as f32 / RUNS as f32) < 0.05,
            "too many failures when guessing keylength"
        );
    }
}
