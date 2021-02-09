// these "mod" statements bring in dict.rs and gen.rs files
mod dict;
mod gen;

// these "use" statements bring the structs into scope
use dict::Dictionary;
use gen::Generator;

fn main() -> anyhow::Result<()> {
    let words = std::fs::read_to_string("words/google-10000-english-usa-no-swears.txt")?;
    let dict = Dictionary::from_string(words);

    println!("{} words in dictionary", dict.len());

    let mut gen = Generator::with_dict(&dict);

    println!("generating 5 sentences with 10 words each...");

    for _ in 0..5 {
        let s = gen.generate_words(10);
        println!("{}", s);
    }

    Ok(())
}
