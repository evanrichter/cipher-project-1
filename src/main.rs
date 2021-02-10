// these "mod" statements bring in ciphers/mod.rs, dict.rs, gen.rs, and utils.rs files
mod ciphers;
mod dict;
mod gen;
mod utils;

// these "use" statements bring the structs into scope
use dict::Dictionary;
use gen::Generator;

// this "use" statement brings the trait into scope so we can use its methods
use ciphers::Cipher;

fn main() -> anyhow::Result<()> {
    let words = std::fs::read_to_string("words/default.txt")?;
    let dict = Dictionary::from_string(words);

    println!("{} words in dictionary", dict.len());

    let mut gen = Generator::with_dict(&dict);

    println!("generating 5 sentences with 10 words each then doing ROT13...\n");

    let rot13 = ciphers::Rot13;
    for _ in 0..5 {
        let plaintext = gen.generate_words(10);
        let ciphertext = rot13.encrypt(&plaintext, &dict);
        let decrypted = rot13.decrypt(&ciphertext, &dict);

        println!("plaintext: {}", plaintext);
        println!("    rot13: {}", ciphertext);
        println!("decrypted: {}", decrypted);
        println!();
    }

    Ok(())
}
