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
    eprintln!("Enter the ciphertext followed by a newline:");

    // read one line from stdin
    let stdin = std::io::stdin();
    let mut ciphertext = String::new();
    stdin.read_line(&mut ciphertext)?;
    ciphertext = ciphertext.trim().to_string();

    eprintln!();
    eprintln!("we read as ciphertext:");
    eprintln!("--------");
    eprintln!("{}", ciphertext);
    eprintln!("--------");

    // 2. crack ciphertext with crack_single_ciphertext()
    let plaintext = crack_single_ciphertext(&ciphertext);

    // 3. print our plaintext guess on stdout
    eprintln!("Resulting plaintext is:");
    eprintln!("--------");
    println!("{}", plaintext);
    eprintln!("--------");

    Ok(())
}
