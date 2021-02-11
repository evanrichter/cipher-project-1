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

### Rng

Also located in `src/gen.rs`.

There is a pseudo-random number generator based on RomuDuo that the `Generator`
uses to pick random words out of the `Dictionary`. It generates `u64` (unsigned
64-bit integers) very quickly.

Right now, only the Generator uses this, so it can stay in gen.rs. But it will
probably be helpful for randomly testing cipher parameters as well, so it could
move out at some point. The idea is that we could create a rotation cipher that
looks like

```rust
struct Rotate {
    rotate_amount: u8,
}
```

Then when we test if `Rotate` even works properly as a cipher, or later if we
can crack any configuration of it, we can generate not only random plaintext to
encrypt, but also random rotation amounts.

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
    fn encrypt<'d>(&self, plaintext: &str, dict: &'d Dictionary) -> String;
    fn decrypt<'d>(&self, ciphertext: &str, dict: &'d Dictionary) -> String;
}
```

This means that in order for an enum, or a struct, or whatever, to implement
Cipher, it must define `encrypt` and `decrypt` functions that match the form
given. One benefit of this is that we can test encryption and decryption
properties generically, with a test like this:

```rust
pub fn test_cipher<T: Cipher>(cipher: T) {
    let dict = Dictionary::from_string("");

    let plaintext = "a man a plan a canal panama";
    let ciphertext = cipher.encrypt(&plaintext, &dict);
    let decrypted = cipher.decrypt(&ciphertext, &dict);

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
and see if the interface was decent to work with. I'm still not sure if
`encrypt` needs access to a dictionary or not, so that might go away.

The `src/ciphers` folder is where the other (and more useful) ciphers will go.

Currently `Rot13` is a struct with no fields. An easy way to extend this cipher
to accomodate _arbitrary_ rotation amount, would be to store this amount in the
struct:

```rust
struct Rot {
    amount: u8,
}
```

Then when it's time to do the rotation (encryption or decryption), the amount to
shift is found by `self.amount`.

## Utils

Located in `src/utils.rs`.

Various handy, yet stand alone utility functions can go here. Future home of
`Rng`? Currently there are two traits here: `NumToChar` and `CharToNum` that
convert between our message space ("a-z ") and numbers (`u8`). These traits let
you use rust native types like `char` and `u8` but extend the functionality.

For example to get the number value of 'b': `'b'.to_num()` which should be 2. Or
to get the character representation of 17: `17.to_char()`. Rust's type system
will make sure you never do something erroneous like `'x'.to_char()` because we
have not implemented `NumToChar` for the char type (and it wouldn't make sense
to do so!). In C, this might be a very hard bug to solve, since C let's you do
the same math on "char" types as you can with "byte" types.
