//! The simple polyalphabetic ciphers
// Vigenere, Beaufort, Beuafort Variant, Nihilist
// variants: autokey, running key, progressive key, multikey

mod vigenere;

pub use self::vigenere::Vigenere;