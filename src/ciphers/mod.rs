//! Various Ciphers

//http://www.cryptoprograms.com/home

// Files with many variants or high complexity
pub mod transposition;
pub mod enigma;
pub mod composite;
pub mod vigenere;
pub mod beaufort;
pub mod tableaux;
pub mod monoalphabetic;
pub mod playfair;

mod checkerboard;
pub use self::checkerboard::StraddlingCheckerboard;

mod polybius;
pub use self::polybius::Polybius;

mod alberti;
pub use self::alberti::Alberti;




