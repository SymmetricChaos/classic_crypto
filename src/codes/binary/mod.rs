//! Various Binary codes.
/// This codes all return a String with 0s and 1s or turn a String of 0s and 1s into something else.

mod ascii;
mod bacon;
mod fibonacci;
mod morse;
mod baudot;
mod unicode;
mod base64;

mod code_generators;
mod binary_code_tests;

pub use self::ascii::ASCII;
pub use self::bacon::Bacon;
pub use self::morse::MorseITU;
pub use self::fibonacci::Fibonacci;
pub use self::baudot::BaudotITA2;
pub use self::unicode::{UTF32,UTF8};
pub use self::base64::Base64;