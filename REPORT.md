# Crytanalysis of a class of ciphers based on character frequency comparison of possible plaintexts using Kasiski Examination

## Introduction

This project was completed by Evan Richter, Matthew Mittelsteadt, and Alex Preneta. 

Tasks completed were as follows:
 * Create example schedulers (All)
   * Created several example schedulers to be able to test with different variations of key scheduling algorithms
 * Key Length Guesser (Evan)
   * Given a ciphertext, generate guesses at possible key lengths to be used in cracking the cipher                                                                 
 * Cracking cipher with key length guesses (Alex/Evan)
   * Given a key length and ciphertext, crack the ciphertext and output the best possible guess based on character frequency comparisons to the known dictionary
 * Spell check the possible plaintexts (Matt)
   * Given a guessed plaintext and a dictionary, spellcheck the guessed plaintext to most closely match actual words in the dictionary.                             
 * Test algorithm (All)
   * Use the scheduling algorithms to create tests for our code and look for opportunities to improve in order to yield better results.                             
 * Write Report (All)
   * Write the report to outline techniques used and our approach                                                                                                           

| Task                                        | Description                                                                                                                                                             | Group Member |
|---------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------|
| Create example schedulers                   | Created several example schedulers to be <br>able to test with different variations of <br>key scheduling algorithms                                                    | All          |
| Key Length Guesser                          | Given a ciphertext, generate guesses at <br>possible key lengths to be used in cracking <br>the cipher                                                                  | Evan         |
| Cracking cipher with <br>key length guesses | Given a key length and ciphertext, crack the <br>ciphertext and output the best possible guess <br>based on character frequency comparisons to the <br>known dictionary | Alex/Evan    |
| Spell check the possible <br>plaintexts     | Given a guessed plaintext and a dictionary, spell<br>check the guessed plaintext to most closely match <br>actual words in the dictionary.                              | Matt         |
| Test algorithm                              | Use the scheduling algorithms to create tests for <br>our code and look for opportunities to improve in <br>order to yield better results.                              | All          |
| Write Report                                | Write the report to outline techniques used and our approach                                                                                                            | All          |

We are submitting a single cryptanalysis approach without modifications made to
the above specifications. This cryptanalysis approach is a variation of the
Kasiski Examination where the ciphertext is broken into `n` slices where `n` is
the guessed length of the key.

## Explanation of Approach
Our approach works in three basic steps: guessing the key length, cracking the
cipher based on the guessed key length, and spellchecking the guesses to more
closely match known plaintexts. In the first step of our approach, the key
length is guess given the ciphertext by dividing the ciphertext into chunks and
calculating the hamming distance between chunks by computing the number of
different bits. Using the hamming distance. After computing the hamming
distance, we are able to generate a guess at the effective key length (length of
the key after a scheduler is ran on it) by calculating the edit distance between
the first keysize worth of bytes.

Once a guess for the keylength is generated, we are able to attempt to crack the
ciphertext by first dividing the ciphertext into  `keylength` slices by taking
every `i`th bit of the keylength and creating a slice with the corrosponding
ciphertext bit. Once the ciphertext is divided into slices, we are able to crack
each slice individually as if it was a single-byte key and calculate the
character frequency of the slice. This character frequency can then be compared
to the histogram generated for the given plaintext dictionary and the closest
match for the slice can be returned. Once every slice is cracked, the slices can
be unsliced into a plaintext using the best character frequency match for each
slice. The resulting plaintext should partially resemble plaintext words from
the original dictionary.

The third step of our approach is to spell-check the guessed plaintext in order
to make it more closely resemble an actual plaintext. This is done by dividing
the guessed plaintext up into slices of possible words. For each slice, the
levenshtein distance is calculated for each possible plaintext word. The best
match is returned. At the conclusion of this step, the best possible guess for
the correct plaintext based on the most closely matching levenshtein distance of
dictionary words is returned to give the final plaintext guess.  This entire
process can be done for the best guess in each instance, or could be ran over
and over to calculate multiple guesses in order to provide more opportunties to
find the correct ciphertext; however, we found in our testing that the top
guesses are generally very closely matched to the original plaintext before
encryption. 

## Description of Approach
The first step in our approach is to guess the potential key lengths given the
ciphertext.  This is done by dividing the ciphertext into chunks and calculating
the hamming distance between the chunks:

```
chunks: Vec<chunks> = ciphertext.divided_into_chunks(chunk size);
let chunk1 = the first chunk
let chunk2 = the second chunk
for chunk1 in chunks:
  for chunk2 in chunks:
    distance += sum(chunk1 ^ chunk2)
```

This process is repeated for all possible key sizes between 3 and 120. The
hamming distance result as well as the keysize guess are pushed to a dictionary
to compare later. Once this dictionary is completed, the keysize guesses are
ranked by normalizing the results and sorting the lowest scored results first.

Once these results are scored, the top result is tried as the effective key
length. Using this length, the cracking algorithm is run. For a given ciphertext
and a key length, the ciphertext is divided into slices:

```
for ct_index, ct_char in ciphertext:
  let bucket = ct_index % keylength
  ct_blocks[bucket].push(ct_char)
```

After slicing the blocks, we attempt to crack each block as a monoalphabetic
shift cipher:

```
for letter in alphabet:
  for block in ct_blocks:
    plaintext = shift block by current letter
    calculate character frequency
    confidence = compare frequency to plaintext dictionary histogram
    CrackResult = {plaintext, confidence)
    return best confidence plaintext for block
  return vector of best confidence plaintexts for each block.
```

This vector result is then unsliced back into a normal plaintext string:

```
for i in range(0, pt_blocks):
  for block in pt_blocks:
    pull out next character in the block
  return the unsliced plaintext
```

The total cracking function returns the plaintext with the best scoring
confidence.

Finally, the plaintext is spellchecked against the plaintext dictionary to
remove words that remain jumbled in order to more accurately present them as
actual plaintext. To accomplish this, we use an implementation of the
levenshtein edit distance:

```
slice plaintext guess into probable words
for slice in plaintext_slices:
  for word in dictionary:
    distance = calculate levenshtein edit distance
    words = push(word, distance)
  best_word = min_distance(words)
full_plaintext_guess = push(best_word)
return full_plaintext_guess
```

The resulting plaintext is the final guess of the original plaintext.
