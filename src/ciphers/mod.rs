//! Various Ciphers


// Files with many variants or high complexity
pub mod transposition;
pub mod enigma;
pub mod composite;


// Easy access certain ciphers use just types "use classic_crypto::ciphers::" followed by the cipher they want
mod monoalphabetic;
pub use self::monoalphabetic::Affine;
pub use self::monoalphabetic::Caesar;
pub use self::monoalphabetic::Substitution;

mod vigenere;
pub use self::vigenere::Vigenere;
pub use self::vigenere::Autokey;

mod playfair;
pub use self::playfair::Playfair;
pub use self::playfair::TwoSquare;
pub use self::playfair::FourSquare;

mod checkerboard;
pub use self::checkerboard::StraddlingCheckerboard;

mod polybius;
pub use self::polybius::Polybius;




