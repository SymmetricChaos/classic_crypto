//! Transposition Ciphers work by creating anagrams of their inputs.

pub mod scytale;
pub mod columnar;
pub mod route;
pub mod rail_fence;

pub use self::columnar::Columnar;
pub use self::scytale::Scytale;