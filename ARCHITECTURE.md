# Architecture

This document explains how the code is laid out and how the pieces fit together.

## Wordlists

Located in `words` folder.

Until we get the official wordlist dictionary, I found a few wordlists that are
reasonable sources of English plaintext. See words/README.md for sources.

`default.txt` is a symlink, currently to the google wordlist.

## Cargo.toml

This file brings in dependencies from the [crates.io](https://crates.io)
repository, and defines some parameters for the project. We won't use it much,
but I do want to bring in [num\_cpus](https://crates.io/crates/num_cpus) and
probably [rayon](https://crates.io/crates/rayon) crates at some point, so when
we start cracking, we can easily go from a single CPU `for` loop to a loop that
divides the work across CPUs almost for free.

## Main

Located in `src/main.rs`.

This is where the main entrypoint into our program lives. It pulls in all the
components into a nice little hierarchy, and then does the things we need it to.
For the final submission, our main function will:

 1. Read in the dictionary wordlist from a file.
 2. Read in ciphertext from stdin (space separated words).
 3. Start cracking the ciphertext using the technique(s) we created.
 4. Output the plaintext on stdout (space separated words).

This mysterious code is at the bottom of main.rs:

```rust
#[cfg(test)]
#[test]
fn test_main() {
    main().expect("main threw an error");
}
```

The `#[cfg(test)]` says "only compile the next thing during `cargo test`". The
`#[test]` says the below function should be run as a test. `main().expect("main
threw an error");` runs `main` and will panic with the message if the `Result`
was an `Error`.

## Dictionary

Located in `src/dict.rs`.

A Dictionary struct manages the master wordlist. It has helper functions to get
the count of words in the list, and build a list from a String (supposedly read
from the wordlist file). `from_str` does not care how the words are separated,
any whitespace will do. This is handy because most wordlist files are newline
separated, not space separated.

## Generator

Located in `src/gen.rs`.

It will be useful to generate pseudo-random plaintext when we want to test our
cracking accuracy.

 1. Load up a wordlist into a `Dictionary`.
 2. Create a new `Generator` that pulls words from this dictionary.
 3. Generate arbitrary random "sentences" of any length you want.
 4. Encipher the plaintext into multiple different ciphertexts.
 5. Ensure that we can crack each ciphertext back to the correct plaintext no
    matter how it was enciphered.
 6. Repeat with varying lengths of plaintext, keylengths, etc., letting it run
    overnight even to try and find a combination that our cracking failed.
 7. Fix the bugs, come up with better cracking algorithms, and repeat.

## Rng

Located in `src/rng.rs`.

This is a pseudo-random number generator based on RomuDuo that the `Generator`
uses to pick random words out of the `Dictionary`. It generates `u64` (unsigned
64-bit integers) very quickly.

The Generator and the Encryptor use this, and later can be used to randomly
select cipher parameters like random keys for testing.

## Cipher trait

Located in `src/ciphers/mod.rs`.

Modules in rust can also be placed in a folder, in a file called mod.rs. The
module here is called `ciphers` and can be accessed from anywhere in the crate
with `use crate::ciphers;`.

I'm told traits in rust are similar to interfaces in other languages. It's a way
to specify the minimal functionality to satisfy some property. Here, I have
defined Cipher as:

```rust
pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}
```

This means that in order for an enum, or a struct, or whatever, to implement
Cipher, it must define `encrypt` and `decrypt` functions that match the form
given. One benefit of this is that we can test encryption and decryption
properties generically, with a test like this:

```rust
pub fn test_cipher<T: Cipher>(cipher: T) {
    let plaintext = "a man a plan a canal panama";
    let ciphertext = cipher.encrypt(&plaintext);
    let decrypted = cipher.decrypt(&ciphertext);

    // plaintext must always differ from ciphertext
    assert_ne!(plaintext, ciphertext);

    // decrypted text must always match original plaintext
    assert_eq!(plaintext, decrypted);

    // plaintext must be shorter or equal to ciphertext length
    assert!(plaintext.len() <= ciphertext.len());
}
```

You can read the function definition above as "function test\_cipher takes a
generic type `T` where `T` implements `Cipher`". That means that inside this
function we can count on being able to call `encrypt` and `decrypt` on the input
parameter `cipher`.

### Cipher Stresstest

Also in `src/ciphers/mod.rs`.

Expanding on the example above, there is a function `stresstest` that takes a
`T: Cipher` and a number of `cycles` to do this test. I've "proven" my Rot13
cipher meets the properties above by testing it with 10,000 cycles every time
`cargo test` is run. And it's fast (<50ms)! Feel free to start with 10,000 and
if the test runs for more than 0.5 seconds, then either make the code faster or
reduce the cycles.

### Rot13

Located in `src/ciphers/rot13.rs`.

To test the usefulness of the `Cipher` trait, I wanted to try a simple cipher
and see if the interface was decent to work with.

The `src/ciphers` folder is where the other (and more useful) ciphers will go.

Currently `Rot13` is a struct with no fields. An easy way to extend this cipher
to accomodate _arbitrary_ rotation amount, would be to store this amount in the
struct:

```rust
struct Rot {
    amount: i8,
}
```

Then when it's time to do the rotation (encryption or decryption), the amount to
shift is found by `self.amount`.

## Encryptor

Located in `src/ciphers/encryptor.rs`.

This struct holds the data necessary to implement the types of ciphers described
in the project paper. It takes a key, a key scheduling algorithm (that we each
will create individually), and then implements Cipher so we can encrypt and
decrypt using the parameters chosen.

So far only encryption is implemented, but decryption is not quite ready. There
are many similarities between the two functions, so I will likely separate out
the common logic to a helper function somehow.

## KeySchedule

Located in `src/ciphers/schedulers/mod.rs`.

This is the trait that defines scheduling keys, and what every team member will
create at least one unique implementation of. An example of a key schedule
algorithm is simply a repeating key:

```rust
// this has no fields because there are no configuration options
pub struct RepeatingKey;

impl super::KeySchedule for RepeatingKey {
    fn schedule(&self, index: usize, key_length: usize, _: usize) -> usize {
        index % key_length
    }
}
```

This example is located in `src/ciphers/schedulers/repeatingkey.rs`.

Everyone will need to make a similar struct and implement `KeySchedule`,
matching the same function parameters and return type. It would be simplest to
just copy repeatingkey.rs into a new file and making changes.

If there are any constants in your keyscheduler, consider making them
configurable. For example, if we wanted to insert a random character every `X`
repetitions of the key, we could make a small change to the previous example:

```rust
// this has one field denoting number of repetitions before random char
pub struct RepeatingKey {
    reps_until_rand: usize,
}

impl super::KeySchedule for RepeatingKey {
    fn schedule(&self, index: usize, key_length: usize, _: usize) -> usize {
        if (index * self.reps_until_rand) % key_length == 0 {
            return usize::MAX; // this will definitely be out of bounds
        } else {
            index % key_length
        }
    }
}
```

## Utils

Located in `src/utils.rs`.

Various handy, yet stand alone utility functions can go here. Currently there
are three helper traits here that mostly help with character <-> number
conversions.

The most handy trait here is `ShiftChar` that is implemented for char types.
The ShiftChar trait gives char a method `shift(self, i8) -> char`. It handles
the shift operation that we need to do during encryption and decryption, so that
we can easily shift forward or backward (negative numbers). An example of using
the shift method in ROT13:

```rust
fn encrypt(&self, plaintext: &str) -> String {
    plaintext.chars().map(|c| c.shift(13)).collect()
}

fn decrypt(&self, ciphertext: &str) -> String {
    ciphertext.chars().map(|c| c.shift(-13)).collect()
}
```

`NumToChar` and `CharToNum` that convert between
our message space ("a-z ") and numbers (`i8`). These traits let you use rust
native types like `char` and `i8` but extend the functionality to match our
encoding scheme.

For example to get the number value of 'b': `'b'.to_num()` which should be 2. Or
to get the character representation of 17: `17.to_char()`. Rust's type system
will make sure you never do something erroneous like `'x'.to_char()` because we
have not implemented `NumToChar` for the char type (and it wouldn't make sense
to do so!). In C, this might be a very hard bug to solve, since C let's you do
the same math on "char" types as you can with "byte" types.

## Crack module

Located in `src/crack/mod.rs`.

This module holds all code related to cracking ciphertexts generated by the
`Encryptor` cipher.

Importantly, when you create a strategy that attempts to crack ciphertext, you
will `impl Crack for MyStrategy` and fill in the function `fn crack(&self,
ciphertext: String) -> Option<CrackResult`.

### Crack trait

Located in `src/crack/mod.rs`.

The function signature for `crack` shows fairly straight-forward inputs, `&self`
and `ciphertext: String`. The output, however is a `CrackResult` wrapped in an
`Option` enum. See the official docs for
[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) for
detailed info, but the gist is that you can have either a `None` result (meaning
no plaintext could be recovered, even a poor try), or `Some(CrackResult { .. })`
was recovered. Some languages use `NULL` for this kind of use case, but in rust
it's more common to separate out the possibilities in separate arms of an enum
like `Option`.

### CrackResult

Located in `src/crack/mod.rs`.

This struct is what is returned by a cracking attempt. It contains the plaintext
recovered, and also a confidence score. See the documentation in the source code
for details.

```rust
struct CrackResult {
    plaintext: String,
    confidence: f64,
}
```
