/// Guess the keylength based on the technique shown in
/// [cryptopals](https://cryptopals.com/sets/1/challenges/6). It is yet to be tested on these shift
/// based ciphers, but this implementation worked against the linked cryptopals challenge based on
/// multi-byte xor.
#[allow(dead_code)]
pub fn keylength_guesses(ciphertext: &[u8]) -> Vec<usize> {
    const KEYSIZE_LO: usize = 4;
    const KEYSIZE_HI: usize = 45;

    let mut keysizes: Vec<(usize, f64)> = Vec::with_capacity(KEYSIZE_HI);

    for keysize in KEYSIZE_LO..KEYSIZE_HI {
        let score = hamming_distance_between_chunks(ciphertext, keysize);
        keysizes.push((keysize, score));
    }

    keysizes.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    for k in keysizes.iter() {
        println!("{}  {}", k.0, k.1);
    }

    keysizes
        .into_iter()
        .take(30)
        .map(|(size, _score)| size)
        .collect()
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
    use crate::rng::FromRng;
    use crate::Cipher;
    use crate::Encryptor;
    use crate::KeySchedule;
    use crate::Rng;

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
        let words = std::fs::read_to_string("words/default.txt").unwrap();
        let dict = crate::Dictionary::from_string(words);
        let mut gen = crate::Generator::with_dict(&dict);
        let plaintext = gen.generate_words(1000);

        // create the encryptor
        let encryptor = Encryptor::new(key, sched, rng);

        // encrypt to ciphertext
        let ciphertext = encryptor.encrypt(&plaintext);

        // get ciphertext into bytes
        let ciphertext = crate::utils::str_to_bytes(&ciphertext);

        // calculate guesses
        dbg!(&expected_keylen);
        let guesses = keylength_guesses(&ciphertext);

        // count how many integer multiples (including exact matches) of the expected keylength are
        // in the top results
        let integer_multiples = guesses
            .iter()
            // only look at the top 5 guesses
            .take(5)
            // filter where the expected keylength is a factor of the guess
            .filter(|&&guess| guess == lcm(guess, expected_keylen))
            // count how many are left
            .count();

        assert!(integer_multiples > 0, "multiple of keylength not in top 5");
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

        // since we are _inserting_ a random char into the key every 9 times, but the underlying
        // key still repeats every 7, I would expect the effective key (the pattern that actually
        // repeats exactly) to be the Least Common Multiple of 7 and 9, or 63
        let expected_keylen = lcm(keylen, period);
        assert_eq!(expected_keylen, 28);

        expected_keylen_rank(keylen, inserted_rand, expected_keylen);
    }

    /// least common multiple
    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    /// greatest common divisor
    fn gcd(a: usize, b: usize) -> usize {
        let mut max = a;
        let mut min = b;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }

    /// stress testing keylength guessing
    #[test]
    fn stresstest() {
        let mut rng = Rng::default();

        // plaintext generator
        let words = std::fs::read_to_string("words/default.txt").unwrap();
        let dict = crate::Dictionary::from_string(words);
        let mut gen = crate::Generator::with_dict(&dict);

        // reusable Vec for key
        let mut key = Vec::new();

        for x in 0..5000 {
            println!("succeeded: {}", x);
            // choose a keylength between 8 and 40
            let keylen = rng.next() % 32 + 8;

            // build the key
            for _ in 0..keylen {
                key.push(rng.next() as u8 as i8);
            }

            // build the plaintext
            let num_words = rng.next() as usize % 256 + 100;
            dbg!(num_words);
            let plaintext = gen.generate_words(num_words);

            // create the encryptor
            // TODO: generate a random scheduler
            let enc_rng = FromRng::from_rng(&mut rng);
            let encryptor = Encryptor::new(key.clone(), RepeatingKey, enc_rng);

            // encrypt to ciphertext
            let ciphertext = encryptor.encrypt(&plaintext);

            // get keylength guesses
            let ciphertext = crate::utils::str_to_bytes(&ciphertext);
            dbg!(keylen);
            let guesses = keylength_guesses(&ciphertext);

            // count how many integer multiples (including exact matches) of the expected keylength are
            // in the top results
            let integer_multiples = guesses
                .iter()
                // only look at the top 5 guesses
                .take(15)
                // filter where the expected keylength is a factor of the guess
                .filter(|&&guess| guess == lcm(guess, keylen as usize))
                // count how many are left
                .count();

            assert!(integer_multiples > 0, "multiple of keylength not in top 5");

            // clear the key contents (but keeps allocation)
            key.clear();
        }
    }
}
