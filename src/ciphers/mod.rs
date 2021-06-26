//! Various Ciphers

//http://www.cryptoprograms.com/home

// Files with many variants or high complexity
pub mod transposition;
pub mod enigma;
pub mod composite;
pub mod tableaux;
pub mod playfair;
pub mod porta;
pub mod m209;
pub mod monoalphabetic;
pub mod polyalphabetic;

mod chaocipher;
pub use self::chaocipher::Chaocipher;

mod tactical;
pub use self::tactical::BATCO;
pub use self::tactical::DRYAD;

mod bazeries;
pub use self::bazeries::Bazeries;
pub use self::bazeries::M94;

mod grille;
pub use self::grille::Grille;

mod checkerboard;
pub use self::checkerboard::StraddlingCheckerboard;
pub use self::checkerboard::VicCheckerboard;

mod polybius;
pub use self::polybius::Polybius;

mod alberti;
pub use self::alberti::Alberti;