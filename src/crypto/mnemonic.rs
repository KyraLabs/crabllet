use bip39::Mnemonic;
use rand::RngCore;

pub enum WordCount {
    Twelve,
    TwentyFour,
}

impl WordCount {
    pub fn entropy_bytes(&self) -> usize {
        match self {
            WordCount::Twelve => 16,
            WordCount::TwentyFour => 32,
        }
    }
}

pub fn generate_mnemonic(word_count: WordCount) -> Result<Mnemonic, bip39::Error> {
    let mut entropy = vec![0u8; word_count.entropy_bytes()];
    rand::rng().fill_bytes(&mut entropy);
    Mnemonic::from_entropy(&entropy)
}
