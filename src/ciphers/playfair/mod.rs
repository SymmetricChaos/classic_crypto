//! Ciphers based on the Playfair Cipher
pub mod playfair;
pub mod two_square;

pub use self::playfair::{Playfair,playfair_from_keyword};