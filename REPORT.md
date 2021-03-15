# Crytanalysis of a class of ciphers based on character frequency comparison of possible plaintexts using Kasiski Examination
## Introduction
This project was completed by Evan Richter, Matthew Mittelsteadt, and Alex Preneta. 
Tasks completed were as follows:
| Task                                        | Description                                                                                                                                                             | Group Member |
|---------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------|
| Create example schedulers                   | Created several example schedulers to be <br>able to test with different variations of <br>key scheduling algorithms                                                    | All          |
| Key Length Guesser                          | Given a ciphertext, generate guesses at <br>possible key lengths to be used in cracking <br>the cipher                                                                  | Evan         |
| Cracking cipher with <br>key length guesses | Given a key length and ciphertext, crack the <br>ciphertext and output the best possible guess <br>based on character frequency comparisons to the <br>known dictionary | Alex/Evan    |
| Spell check the possible <br>plaintexts     | Given a guessed plaintext and a dictionary, spell<br>check the guessed plaintext to most closely match <br>actual words in the dictionary.                              | Matt         |
| Test algorithm                              | Use the scheduling algorithms to create tests for <br>our code and look for opportunities to improve in <br>order to yield better results.                              | All          |
| Write Report                                | Write the report to outline techniques used and our approach                                                                                                            | All          |

We are submitting a single cryptanalysis approach without modifications made to 
the above specifications. This cryptanalysis approach is a variation of the Kasiski
Examination wehre the ciphertext is broken into n slices where n is the guessed length
of the key.

## Explanation of Approach
Our approach works in three basic steps: guessing the key length, cracking the cipher 
based on the guessed key length, and spellchecking the guesses to more closely match 
known plaintexts. In the first step of our approach, the key length is guess given 
the ciphertext by dividing the ciphertext into chunks and calculating the hamming 
distance between chunks by computing the number of different bits. Using the hamming
distance. After computing the hamming distance, we are able to generate a guess at the
effective key length (length of the key after a scheduler is ran on it) by calculating
the edit distance between the first keysize worth of bytes.
Once a guess for the keylength is generated, we are able to attempt to crack the ciphertext
by first dividing the ciphertext into  `keylength` slices by taking every ith bit of the 
keylength and creating a slice with the corrosponding ciphertext bit. Once the ciphertext is 
divided into slices, we are able to crack each slice individually as if it was a single-byte
key and calculate the character frequency of the slice. This character frequency can then be 
compared to the histogram generated for the given plaintext dictionary and the closest match 
for the slice can be returned. Once every slice is cracked, the slices can be unsliced into a
plaintext using the best character frequency match for each slice. The resulting plaintext 
should partially resemble plaintext words from the original dictionary.
The third step of our approach is to spell-check the guessed plaintext in order to make
it more closely resemble an actual plaintext. This is done by dividing the guessed plaintext
up into slices of possible words. For each slice, the levenshtein distance is calculated 
for each possible plaintext word. The best match is returned. At the conclusion of this step,
the best possible guess for the correct plaintext based on the most closely matching levenshtein
distance of dictionary words is returned to give the final plaintext guess. 
This entire process can be done for the best guess in each instance, or could be ran over and over
to calculate multiple guesses in order to provide more opportunties to find the correct 
ciphertext; however, we found in our testing that the top guesses are generally very closely
matched to the original plaintext before encryption. 
