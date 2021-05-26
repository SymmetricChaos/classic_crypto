//! Various Codes

mod binary;
mod spelling_alphabet;
mod godel;
mod japanese;

pub use self::spelling_alphabet::SpellingAlphabet;
pub use self::binary::{Bacon,BaudotITA2,Fibonacci,MorseITU,UTF32,UTF8};
pub use self::godel::Godel;
pub use self::japanese::{NihonShikiHiragana,NihonShikiKatakana};