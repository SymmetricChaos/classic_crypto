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
pub mod porta;
pub mod m209;

mod chaocipher;
pub use self::chaocipher::Chaocipher;

mod tactical;
pub use self::tactical::BATCO;
pub use self::tactical::DRYAD;

mod bazeries;
pub use self::bazeries::Bazeries;

mod grille;
pub use self::grille::Grille;

mod checkerboard;
pub use self::checkerboard::StraddlingCheckerboard;
pub use self::checkerboard::VicCheckerboard;

mod polybius;
pub use self::polybius::{Polybius,GenPolybius};

mod alberti;
pub use self::alberti::Alberti;

mod polyalphabetic;
pub use self::polyalphabetic::Vigenere;