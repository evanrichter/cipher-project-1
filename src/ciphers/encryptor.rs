use crate::ciphers::schedulers::RepeatingKey;
use crate::ciphers::{Cipher, KeySchedule};
use crate::rng::{FromRng, Rng};
use crate::utils::{reduce_key, Key, NumToChar, ShiftChar};

use std::cell::Cell;
use std::fmt::Debug;

/// The main encryption scheme described in the project description
#[derive(Debug)]
pub struct Encryptor<K: KeySchedule + Debug> {
    /// The key chosen for this encryptor.
    ///
    /// The key length is called `t` in the description and is guaranteed to be between 1 and 24.
    key: Key,
    /// The scheduling algorithm for this encryptor
    keyschedule: K,
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

impl<K: KeySchedule + Debug> Encryptor<K> {
    /// Create a new Encryptor configured with the given key, [`KeySchedule`], and [`Rng`].
    #[allow(dead_code)]
    pub fn new(mut key: Key, keyschedule: K, rng: Rng) -> Self {
        reduce_key(&mut key);
        Self {
            key,
            keyschedule,
            rng,
            prev_plaintext_length: Cell::new(None),
        }
    }
}

impl Encryptor<RepeatingKey> {
    /// Encryptor with a simple repeating key scheduler.
    pub fn repeating_key(mut key: Key, rng: Rng) -> Self {
        reduce_key(&mut key);
        Self {
            key,
            keyschedule: RepeatingKey,
            rng,
            prev_plaintext_length: Cell::new(None),
        }
    }
}

impl<K: KeySchedule + Debug> Cipher for Encryptor<K> {
    fn encrypt(&self, plaintext: &str) -> String {
        // get keylen and plaintext len
        let keylen = self.key.len();
        let ptlen = plaintext.len();

        // stash the plaintext length in our "side channel" and also assert that we don't encrypt
        // two things in a row
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
            // get key index to use as shift.
            let index = self.keyschedule.schedule(cipher.len(), keylen, ptlen);

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

    fn decrypt(&self, ciphertext: &str) -> String {
        // get keylen
        let keylen = self.key.len();

        // get plaintext length over our "side channel", replacing with None
        let ptlen = self
            .prev_plaintext_length
            .replace(None)
            .expect("encrypt must be called before decrypt");

        // create an empty string to fill with plaintext
        let mut plaintext = String::new();

        // read every byte of ciphertext
        'decryption: for (index, cipher) in ciphertext.chars().enumerate() {
            // get key index to use as shift.
            let index = self.keyschedule.schedule(index, keylen, ptlen);

            // get the shift amount from the key, or discard the character if the character was
            // generated randomly.
            let shift = match self.key.get(index) {
                Some(s) => *s,
                None => continue 'decryption,
            };

            // apply the shift amount in reverse because we are decrypting not encrypting.
            let plain_char = cipher.shift(-shift);

            // push the decrypted character into plaintext string
            plaintext.push(plain_char);
        }

        // return plaintext
        plaintext
    }
}

impl<K: KeySchedule + Debug + FromRng> FromRng for Encryptor<K> {
    fn from_rng(rng: &mut Rng) -> Self {
        // generate a friendly key
        let key = FromRng::from_rng(rng);

        // generate a keyschedule from rng
        let keyschedule = K::from_rng(rng);

        // spin off another rng from this one
        let rng = FromRng::from_rng(rng);

        Self {
            key,
            keyschedule,
            rng,
            prev_plaintext_length: Cell::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ciphers::testing::stresstest;

    #[test]
    fn repeating_key_stress() {
        let encryptor = Encryptor::repeating_key(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Rng::default());
        stresstest(encryptor, 10000).unwrap();
    }
}
