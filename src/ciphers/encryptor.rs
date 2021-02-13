use crate::ciphers::{Cipher, KeySchedule};
use crate::rng::Rng;
use crate::utils::{NumToChar, ShiftChar};

use std::cell::Cell;

/// The main encryption scheme described in the project description
pub struct Encryptor<'k> {
    /// The key chosen for this encryptor.
    ///
    /// The key length is called `t` in the description and is guaranteed to be between 1 and 24.
    key: &'k [i8],
    /// The scheduling algorithm for this encryptor
    keyschedule: KeySchedule,
    /// Rng to insert random characters when needed
    rng: Rng,
    /// The length of the plaintext most recently encrypted, or `None` if no plaintext was
    /// encrypted yet.
    ///
    /// From the professor:
    /// > In any such scheme, sender and recipient share a secret key, the scheme algorithms and
    /// > various scheme parameters, including plaintext/ciphertext/key lengths.
    ///
    /// The reason this is a [`Cell`]: The ciphertext length can be measured on receipt, and the
    /// key length can be derived from `self`, but the recipient must also know the length of the
    /// plaintext before decrypting. This doesn't fit the model of our [`Cipher`] trait very well,
    /// so we use this Cell type as sort of a workaround. Since sending the plaintext length to the
    /// recipient is literally a side-channel, we use the Cell type as a side-channel around the
    /// immutable `&self`. Cell lets us mutate the contained value even if we don't have a mutable
    /// reference.
    prev_plaintext_length: Cell<Option<usize>>,
}

impl<'k> Encryptor<'k> {
    /// Create a new Encryptor configured with the given key, [`KeySchedule`], and [`Rng`].
    #[allow(dead_code)]
    pub fn new(key: &'k [i8], keyschedule: KeySchedule, rng: Rng) -> Self {
        Self {
            key,
            keyschedule,
            rng,
            prev_plaintext_length: Cell::new(None),
        }
    }

    /// Encryptor with a simple repeating key scheduler.
    ///
    /// This key scheduler simply takes the key, and cycles through it, start to finish.
    ///
    /// Example with key `HEADCRAB` and plaintext: `RISE AND SHINE MISTER FREEMAN RISE AND SHINE`:
    ///
    /// ```
    ///  Plaintext:     RISE AND SHINE MISTER FREEMAN RISE AND SHINE
    /// Shifted by:     HEADCRABHEADCRABHEADCRABHEADCRABHEADCRABHEAD
    /// ```
    pub fn repeating_key(key: &'k [i8], rng: Rng) -> Self {
        fn keyschedule(index: usize, key_length: usize, _: usize) -> usize {
            index % key_length
        }

        Self {
            key,
            keyschedule: &keyschedule,
            rng,
            prev_plaintext_length: Cell::new(None),
        }
    }
}

impl<'k> Cipher for Encryptor<'k> {
    fn encrypt(&self, plaintext: &str) -> String {
        // get keylen and plaintext len
        let keylen = self.key.len();
        let ptlen = plaintext.len();

        // assert that we don't encrypt two things in a row
        assert!(
            self.prev_plaintext_length.replace(Some(ptlen)).is_none(),
            "must decrypt after encrypt"
        );

        // clone out the rng from self (otherwise decrypting will not start from the same rng
        // state!)
        let mut rng = self.rng.clone();

        // create an iterator over the plaintext
        let mut plaintext = plaintext.chars().peekable();

        // the encrypted string to return
        let mut cipher = String::new();

        // continue encryption as long as there is a plaintext character left to read
        'encryption: while plaintext.peek().is_some() {
            // get key index to use as shift
            let index = (self.keyschedule)(cipher.len(), keylen, ptlen);

            // get the shift amount from the key, or insert a random character. A random character
            // is only inserted when the index is out of bounds of the key.
            let shift = match self.key.get(index) {
                Some(s) => *s,
                None => {
                    // get a random number and wrap it to the correct range
                    let rand = rng.next() as i8;
                    // push the character to the ciphertext
                    cipher.push(rand.to_char());
                    continue 'encryption;
                }
            };

            // apply the shift amount to the next plaintext char. unwrap will always succeed
            // because we "peeked" the iterator at the beginning of the loop already.
            let cipher_char = plaintext.next().unwrap().shift(shift);

            // push the enciphered character to the cipher string
            cipher.push(cipher_char);
        }

        // return the ciphertext
        cipher
    }

    fn decrypt(&self, _ciphertext: &str) -> String {
        let _ptlen = self
            .prev_plaintext_length
            .replace(None)
            .expect("encrypt must be called before decrypt");
        String::from("asdf")
    }
}
