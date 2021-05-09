//! Various Ciphers


// Files with many variants or high complexity
pub mod transposition;
pub mod enigma;
pub mod composite;


// Easy access to certain ciphers. User just types "use classic_crypto::ciphers::" followed by the cipher they want
mod monoalphabetic;
pub use self::monoalphabetic::Affine;
pub use self::monoalphabetic::Caesar;
pub use self::monoalphabetic::Substitution;
pub use self::monoalphabetic::DecoderRing;

pub mod vigenere;

mod beaufort;
pub use self::beaufort::{Beaufort,BeaufortAutokey,BeaufortRunningKey};


mod playfair;
pub use self::playfair::Playfair;
pub use self::playfair::TwoSquare;
pub use self::playfair::FourSquare;

mod checkerboard;
pub use self::checkerboard::StraddlingCheckerboard;

mod polybius;
pub use self::polybius::Polybius;

mod alberti;
pub use self::alberti::Alberti;




