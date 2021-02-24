/// Guess the keylength based on the technique shown in
/// [cryptopals](https://cryptopals.com/sets/1/challenges/6). It is yet to be tested on these shift
/// based ciphers, but this implementation worked against the linked cryptopals challenge based on
/// multi-byte xor.
#[allow(dead_code)]
pub fn guesses(ciphertext: &[u8]) -> Vec<usize> {
    const KEYSIZE_LO: usize = 4;
    const KEYSIZE_HI: usize = 64;

    let mut keysizes: Vec<(usize, f64)> = Vec::new();

    for keysize in KEYSIZE_LO..KEYSIZE_HI {
        let score = hamming_distance_between_chunks(ciphertext, keysize);
        keysizes.push((keysize, score));
    }

    keysizes.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    keysizes
        .into_iter()
        .take(30)
        .map(|(size, _score)| size)
        .collect()
}

/// Take 4 chunks of size `chunksize` and calculate the Hamming distance between each chunk.
pub fn hamming_distance_between_chunks(input: &[u8], chunksize: usize) -> f64 {
    const CHUNKS: usize = 4;
    let chunks: Vec<&[u8]> = input.chunks_exact(chunksize).take(CHUNKS).collect();
    let mut distance = 0f64;
    for ii in 0..CHUNKS {
        for jj in ii..CHUNKS {
            distance += hamming_distance(chunks[ii], chunks[jj]) as f64;
        }
    }

    distance / chunksize as f64
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
