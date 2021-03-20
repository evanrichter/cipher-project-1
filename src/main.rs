// these "mod" statements bring in ciphers/mod.rs, dict.rs, gen.rs, and utils.rs files
mod ciphers;
mod crack;
mod dict;
mod gen;
mod rng;
mod utils;

use crack::crack_single_ciphertext;

fn main() -> anyhow::Result<()> {
    // 1. get ciphertext from stdin
    println!("Enter the ciphertext followed by a newline:");

    // read one line from stdin
    let stdin = std::io::stdin();
    let mut ciphertext = String::new();
    stdin.read_line(&mut ciphertext)?;
    ciphertext = ciphertext.trim().to_string();

    // 2. crack ciphertext with crack_single_ciphertext()
    let plaintext = crack_single_ciphertext(&ciphertext);

    // 3. print our plaintext guess on stdout
    println!("Resulting plaintext is:\n");
    println!("{}", plaintext);

    Ok(())
}
