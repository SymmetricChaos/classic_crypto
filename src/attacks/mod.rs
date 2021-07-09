//! Attacks on certain ciphers

//http://www.cryptoprograms.com/home

mod attack_tests;

pub mod score_text;
pub mod ngram_data_extract;

mod caesar_attack;
pub use self::caesar_attack::caesar_attack;

mod scytale_attack;
pub use self::scytale_attack::scytale_attack;

mod vigenere_attack;
pub use self::vigenere_attack::kasiski;