// these "mod" statements bring in ciphers/mod.rs, dict.rs, gen.rs, and utils.rs files
mod ciphers;
mod crack;
mod dict;
mod gen;
mod rng;
mod utils;

// these "use" statements bring the structs into scope
pub use ciphers::Encryptor;
pub use dict::Dictionary;
pub use gen::Generator;
pub use rng::Rng;

// this "use" statement brings the traits into scope so we can use their methods
pub use ciphers::{Cipher, KeySchedule};

fn main() -> anyhow::Result<()> {
    let mut words = std::fs::read_to_string("words/default.txt")?;
    let dict = Dictionary::from_string(&mut words);

    println!("{} words in dictionary", dict.len());

    let mut gen = Generator::with_dict(&dict);

    println!("generating 5 sentences with 10 words each then encrypting...\n");

    let cipher = ciphers::Encryptor::repeating_key(vec![0, 1, -1], Rng::default());
    for _ in 0..5 {
        let plaintext = gen.generate_words(10);
        let ciphertext = cipher.encrypt(&plaintext);
        let decrypted = cipher.decrypt(&ciphertext);

        println!("plaintext: {}", plaintext);
        println!("encrypted: {}", ciphertext);
        println!("decrypted: {}", decrypted);
        println!();
    }

    Ok(())
}

#[cfg(test)]
#[test]
fn test_main() {
    main().expect("main threw an error");
}
