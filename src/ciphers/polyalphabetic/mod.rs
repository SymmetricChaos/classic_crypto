//! Polyalphabetic ciphers and variants

mod polyalphabetic_tests;

mod vigenere;
mod beaufort;
mod beaufort_variant;

pub use self::vigenere::Vigenere;
pub use self::beaufort::Beaufort;
pub use self::beaufort_variant::BeaufortVariant;


mod autokey;
pub use self::autokey::Autokey;