# Architecture

This document explains how the code is laid out and how the pieces fit together.

## Main

Located in `src/main.rs`.

This is where the main entrypoint into our program lives. It pulls in all the
components into a nice little hierarchy, and then does the things we need it to.
For the final submission, our main function will:

 1. Read in the dictionary wordlist from a file.
 2. Read in ciphertext from stdin (space separated words).
 3. Start cracking the ciphertext using the technique(s) we created.
 4. Output the plaintext on stdout (space separated words).

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
