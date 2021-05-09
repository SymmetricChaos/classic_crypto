//! Various Ciphers

//http://www.cryptoprograms.com/home

// Files with many variants or high complexity
pub mod transposition;
pub mod enigma;
pub mod composite;
pub mod vigenere;
pub mod beaufort;


// Easy access to certain ciphers. User just types "use classic_crypto::ciphers::" followed by the cipher they want
mod monoalphabetic;
pub use self::monoalphabetic::{Affine,Caesar,Substitution,DecoderRing};

mod playfair;
pub use self::playfair::{Playfair,TwoSquare,TwoSquareInverting,FourSquare};

mod checkerboard;
pub use self::checkerboard::StraddlingCheckerboard;

mod polybius;
pub use self::polybius::Polybius;

mod alberti;
pub use self::alberti::Alberti;




