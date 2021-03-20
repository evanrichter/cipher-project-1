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

    // 2. crack ciphertext with crack_single_ciphertext()

    // 3. print our plaintext guess on stdout

    Ok(())
}
