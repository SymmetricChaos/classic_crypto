//! Various Codes

mod binary;
mod spelling_alphabet;
mod godel;

pub use self::spelling_alphabet::SpellingAlphabet;
pub use self::binary::{Bacon,BaudotITA2,Fibonacci,MorseITU,UTF32};
pub use self::godel::Godel;