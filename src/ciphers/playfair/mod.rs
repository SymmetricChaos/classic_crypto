//! Ciphers based on the Playfair Cipher
mod playfair;
mod two_square;
mod four_square;
mod playfair_tests;

pub use self::playfair::Playfair;
pub use self::two_square::{TwoSquare,TwoSquareInverting};
pub use self::four_square::FourSquare;