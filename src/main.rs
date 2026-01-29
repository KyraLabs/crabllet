mod crypto;

use crypto::{generate_mnemonic, WordCount};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let word_count = match args.get(1).map(|s| s.as_str()) {
        Some("24") => WordCount::TwentyFour,
        _ => WordCount::Twelve,
    };

    let mnemonic = generate_mnemonic(word_count).expect("Failed to generate mnemonic");

    println!("Mnemonic: {}", mnemonic);
}